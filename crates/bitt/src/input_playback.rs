use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::asserts::AsserterPlugin;

mod artefact_paths;
mod frame_metrics;
mod playback;
mod recording;

#[derive(Debug, Resource)]
struct FirstUpdate(Duration);
#[derive(Debug, Clone, Copy, Event)]
struct TestQuitEvent(bool);
#[derive(Serialize, Deserialize, Debug, Clone, Default, Resource)]
struct TestScript {
    events: Vec<(Duration, UserInput)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UserInput {
    KeyPress(KeyCode),
    KeyRelese(KeyCode),
    MouseButtonPress(MouseButton),
    MouseButtonRelease(MouseButton),
    ControllerAxisChange(GamepadAxis, f32),
    ControllerButtonPress(GamepadButton),
    ControllerButtonRelease(GamepadButton),
    MouseScroll(MouseWheel),
    MouseMove(Vec2),
    Quit,
}

/// Options to use when running playback testing.
/// Inserted as a resource for test gear usage, you shouldn't modify it
#[derive(Debug, Resource, Clone)]
pub struct PlaybackTestingOptions {
    /// If true, the test will panic if the script doesn't exist.
    pub read_only: bool,
    /// The amount of seconds to wait for the asserter to pass after input ends.
    pub assert_window: f32,
    /// If true, the test will collect frame metrics. There may be some overhead from the test gear
    /// Some operations are performed sync and may create a long frame, but it should only land on one frame.
    pub collect_frame_metrics: bool,
}

impl Default for PlaybackTestingOptions {
    fn default() -> Self {
        Self {
            read_only: false,
            assert_window: 5.0,
            collect_frame_metrics: true,
        }
    }
}

/// Plugin that once inserted will perform playback testing.
#[derive(Debug)]
pub struct PlaybackTestGear {
    case_name: String,
    options: PlaybackTestingOptions,
}

impl PlaybackTestGear {
    /// Creates a new instance of the plugin.
    /// `case_name` is the name of the test case to run.
    /// `options` are the options to use when running the test.
    pub fn new(case_name: String, options: PlaybackTestingOptions) -> Self {
        Self { case_name, options }
    }
}

impl Plugin for PlaybackTestGear {
    fn build(&self, app: &mut App) {
        let (script_path, artefact_path) = get_paths(self.case_name.clone());

        if let Some(script) = load_script(&script_path) {
            if self.options.collect_frame_metrics {
                app.add_plugins(frame_metrics::FrameMetricPlugin);
            }

            app.add_plugins(playback::PlaybackPlugin {
                script,
                artefact_path,
            })
        } else {
            assert!(
                !self.options.read_only,
                "Script {} doesn't exist",
                self.case_name
            );

            app.add_plugins(recording::RecordingPlugin { script_path })
        }
        .add_systems(First, set_first_update)
        .insert_resource(self.options.clone())
        .add_plugins(AsserterPlugin);
    }
}

fn set_first_update(
    mut commands: Commands,
    time: Res<Time<Real>>,
    first_update: Option<Res<FirstUpdate>>,
) {
    if first_update.is_none() {
        commands.insert_resource(FirstUpdate(time.elapsed()));
    }
}

fn get_paths(case_name: String) -> (PathBuf, PathBuf) {
    let base_path = Path::new("bitt");

    let script_path = base_path
        .join("test_scripts")
        .join(format!("{}.bitt_script", case_name));

    let artefact_path = base_path.join("artefacts").join(case_name);

    (script_path, artefact_path)
}

fn load_script(path: &Path) -> Option<TestScript> {
    if path.exists() {
        let script = read_to_string(path).unwrap();
        let script: TestScript = serde_json::from_str(&script).unwrap();
        Some(script)
    } else {
        None
    }
}
