use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::asserts::AsserterPlugin;

mod playback;
mod recording;

#[derive(Debug, Resource)]
struct FirstUpdate(Duration);

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

/// Plugin that once inserted will perform playback testing.
#[derive(Debug)]
pub struct PlaybackTestGear {
    case_name: String,
    read_only: bool,
}

impl PlaybackTestGear {
    /// Creates a new instance of the plugin.
    /// `case_name` is the name of the test case to run.
    /// `read_only` is whether the test should be run in read-only mode. Useful for CI so you don't accidentally wait for input.
    /// If `read_only` is `true` and the test script doesn't exist, the test will fail.
    pub fn new(case_name: String, read_only: bool) -> Self {
        Self {
            case_name,
            read_only,
        }
    }
}

impl Plugin for PlaybackTestGear {
    fn build(&self, app: &mut App) {
        let (script_path, artefact_path) = get_paths(self.case_name.clone());

        if let Some(script) = load_script(&script_path) {
            app.add_plugins(playback::PlaybackPlugin {
                script,
                artefact_path,
            })
        } else {
            assert!(!self.read_only, "Script {} doesn't exist", self.case_name);

            app.add_plugins(recording::RecordingPlugin { script_path })
        }
        .add_systems(First, set_first_update)
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
