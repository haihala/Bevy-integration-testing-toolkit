use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, File},
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use bevy::{
    app::AppExit,
    ecs::system::SystemId,
    prelude::*,
    render::view::screenshot::ScreenshotManager,
    window::{exit_on_all_closed, exit_on_primary_closed, PrimaryWindow},
};

use crate::{AssertSystem, Asserter};

#[derive(Debug, Resource)]
struct ScriptPath(PathBuf);

#[derive(Debug, Resource)]
struct ArtefactPath(PathBuf);

#[derive(Debug)]
pub struct PlaybackTestGear {
    case_name: String,
    read_only: bool,
}

impl PlaybackTestGear {
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
            let id = app.world.register_system(record_artefacts);
            app.insert_resource(script)
                .add_systems(First, (set_first_update, script_player).chain())
                .add_event::<CustomQuitEvent>()
                .insert_resource(QuitCallback(id))
                .add_systems(Update, delayed_exit)
                .insert_resource(ArtefactPath(artefact_path))
        } else {
            assert!(!self.read_only, "Script {} doesn't exist", self.case_name);

            app.insert_resource(TestScript::default())
                .add_systems(First, (set_first_update, script_recorder).chain())
                .add_systems(
                    PostUpdate,
                    save_script
                        .run_if(on_event::<AppExit>())
                        .after(exit_on_primary_closed)
                        .after(exit_on_all_closed),
                )
        }
        .insert_resource(ScriptPath(script_path));
    }
}

#[derive(Debug, Resource)]
struct FirstUpdate(Duration);

fn set_first_update(
    mut commands: Commands,
    time: Res<Time<Real>>,
    first_update: Option<Res<FirstUpdate>>,
) {
    if first_update.is_none() {
        commands.insert_resource(FirstUpdate(time.elapsed()));
    }
}

#[derive(Debug, Resource)]
struct QuitCallback(SystemId);

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UserInput {
    KeyPress(KeyCode),
    KeyRelese(KeyCode),
    Quit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resource)]
struct TestScript(Vec<(Duration, UserInput)>);

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

fn script_recorder(
    mut script: ResMut<TestScript>,
    time: Res<Time<Real>>,
    input: Res<Input<KeyCode>>,
    start_time: Res<FirstUpdate>,
) {
    let timestamp = time.elapsed() - start_time.0;

    for key in input.get_just_pressed() {
        script.0.push((timestamp, UserInput::KeyPress(*key)));
    }

    for key in input.get_just_released() {
        script.0.push((timestamp, UserInput::KeyRelese(*key)));
    }
}

fn save_script(
    script: Res<TestScript>,
    path: Res<ScriptPath>,
    time: Res<Time<Real>>,
    first_update: Option<Res<FirstUpdate>>,
) {
    let Some(start_time) = first_update else {
        return;
    };

    let mut script = script.clone();
    script
        .0
        .push((time.elapsed() - start_time.0, UserInput::Quit));

    let prefix = path.0.parent().unwrap();
    create_dir_all(prefix).unwrap();
    let file = File::create(&path.0).unwrap();
    serde_json::to_writer(file, &script).unwrap();
}

#[derive(Debug, Clone, Copy, Event)]
struct CustomQuitEvent;

fn script_player(
    mut last_run: Local<Duration>,
    time: Res<Time<Real>>,
    script: Res<TestScript>,
    mut quit_events: EventWriter<CustomQuitEvent>,
    mut kb_input: ResMut<Input<KeyCode>>,
    first_update: Option<Res<FirstUpdate>>,
) {
    let Some(start_time) = first_update else {
        return;
    };

    for ev in script
        .0
        .iter()
        .skip_while(|(event_time, _)| *event_time + start_time.0 <= *last_run)
        .take_while(|(event_time, _)| *event_time + start_time.0 <= time.elapsed())
        .map(|(_, input)| input)
    {
        match ev {
            UserInput::KeyPress(key) => kb_input.press(*key),
            UserInput::KeyRelese(key) => kb_input.release(*key),
            UserInput::Quit => quit_events.send(CustomQuitEvent),
        }
    }

    *last_run = time.elapsed();
}

#[derive(Debug, Clone, Resource)]
struct EndScreenshot(PathBuf);

fn record_artefacts(
    mut commands: Commands,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    path: Res<ArtefactPath>,
) {
    if path.0.exists() {
        remove_dir_all(path.0.clone()).unwrap();
    }
    create_dir_all(path.0.clone()).unwrap();
    let img_path = path.0.clone().join("screenshot.png");
    screenshot_manager
        .save_screenshot_to_disk(main_window.single(), img_path.clone())
        .unwrap();

    commands.insert_resource(EndScreenshot(img_path));
}

#[allow(clippy::too_many_arguments)]
fn delayed_exit(
    mut quit_events: ResMut<Events<AppExit>>,
    mut commands: Commands,
    callback: Res<QuitCallback>,
    assert_sys: Res<AssertSystem>,
    custom_quit_events: EventReader<CustomQuitEvent>,
    mut started: Local<bool>,
    screenshot: Option<Res<EndScreenshot>>,
    asserter: Res<Asserter>,
) {
    if *started {
        let mut done = false;

        if let Some(shot) = screenshot {
            if shot.0.exists()
                && File::open(shot.0.clone())
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .len()
                    > 0
            {
                done = true;
            }
        }

        if done && asserter.ran {
            if asserter.passed {
                println!("Test passed");
                quit_events.send(AppExit);
            } else {
                // TODO: figure out a nicer way to fail the test
                panic!("Test failed");
            }
        }
    } else if !custom_quit_events.is_empty() {
        commands.run_system(assert_sys.0);
        commands.run_system(callback.0);
        *started = true;
    }
}
