use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, File},
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use bevy::{
    app::AppExit,
    ecs::system::SystemId,
    input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent, GamepadInfo},
    prelude::*,
    render::view::screenshot::ScreenshotManager,
    utils::{hashbrown::HashSet, HashMap},
    window::PrimaryWindow,
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
                .add_systems(
                    First,
                    (set_first_update, connect_pads, script_player).chain(),
                )
                .insert_resource(ArtefactPath(artefact_path))
                .insert_resource(QuitCallback(id))
                .add_event::<StartAsserting>()
                .add_event::<TestQuitEvent>()
                .add_systems(Update, (run_asserts, delayed_exit).chain())
        } else {
            assert!(!self.read_only, "Script {} doesn't exist", self.case_name);

            app.insert_resource(TestScript::default())
                .add_systems(
                    First,
                    (set_first_update, script_recorder, recording_asserter).chain(),
                )
                .add_event::<SaveQuitEvent>()
                .add_systems(PostUpdate, save_script.run_if(on_event::<SaveQuitEvent>()))
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

#[derive(Debug, Clone, Copy, Event)]
struct SaveQuitEvent;
#[derive(Debug, Clone, Copy, Event)]
struct TestQuitEvent(bool);

#[derive(Debug, Clone, Copy, Event)]
struct StartAsserting;

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
    Quit,
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

#[allow(clippy::too_many_arguments)]
fn script_recorder(
    mut script: ResMut<TestScript>,
    time: Res<Time<Real>>,
    first_update: Option<Res<FirstUpdate>>,
    input: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    pad_buttons: Res<Input<GamepadButton>>,
    axis: Res<Axis<GamepadAxis>>,
    mut axis_cache: Local<HashMap<GamepadAxis, f32>>,
) {
    let Some(start_time) = first_update else {
        return;
    };

    let timestamp = time.elapsed() - start_time.0;

    for key in input.get_just_pressed() {
        script.events.push((timestamp, UserInput::KeyPress(*key)));
    }

    for key in input.get_just_released() {
        script.events.push((timestamp, UserInput::KeyRelese(*key)));
    }

    for button in mouse_buttons.get_just_pressed() {
        script
            .events
            .push((timestamp, UserInput::MouseButtonPress(*button)));
    }

    for button in mouse_buttons.get_just_released() {
        script
            .events
            .push((timestamp, UserInput::MouseButtonRelease(*button)));
    }

    for button in pad_buttons.get_just_pressed() {
        script
            .events
            .push((timestamp, UserInput::ControllerButtonPress(*button)));
    }

    for button in pad_buttons.get_just_released() {
        script
            .events
            .push((timestamp, UserInput::ControllerButtonRelease(*button)));
    }

    if axis.is_changed() {
        for dev in axis.devices() {
            let Some(value) = axis.get(*dev) else {
                continue;
            };

            if axis_cache
                .get(dev)
                .map(|cached| (value - cached).abs() > 0.01)
                .unwrap_or(true)
            {
                axis_cache.insert(*dev, value);
                script
                    .events
                    .push((timestamp, UserInput::ControllerAxisChange(*dev, value)));
            }
        }
    }
}

fn recording_asserter(
    mut commands: Commands,
    asserter: ResMut<Asserter>,
    mut quit_events: EventWriter<SaveQuitEvent>,
    assert_sys: Res<AssertSystem>,
) {
    if asserter.passed {
        quit_events.send(SaveQuitEvent);
    } else {
        commands.run_system(assert_sys.0);
    }
}

fn save_script(
    script: Res<TestScript>,
    path: Res<ScriptPath>,
    time: Res<Time<Real>>,
    first_update: Option<Res<FirstUpdate>>,
    mut quit_events: ResMut<Events<AppExit>>,
) {
    let Some(start_time) = first_update else {
        return;
    };

    let mut script = script.clone();
    script
        .events
        .push((time.elapsed() - start_time.0, UserInput::Quit));

    let prefix = path.0.parent().unwrap();
    create_dir_all(prefix).unwrap();
    let file = File::create(&path.0).unwrap();
    serde_json::to_writer(file, &script).unwrap();
    quit_events.send(AppExit);
}

// The point of this is to fake that the pads being used by the inputs are connected.
fn connect_pads(
    script: Res<TestScript>,
    mut events: EventWriter<GamepadEvent>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    for pad in script
        .events
        .iter()
        .filter_map(|(_, input)| match input {
            UserInput::ControllerAxisChange(axis, _) => Some(axis.gamepad),
            UserInput::ControllerButtonPress(button) => Some(button.gamepad),
            UserInput::ControllerButtonRelease(button) => Some(button.gamepad),
            _ => None,
        })
        .collect::<HashSet<_>>()
        .into_iter()
    {
        events.send(GamepadEvent::Connection(GamepadConnectionEvent {
            gamepad: pad,
            connection: GamepadConnection::Connected(GamepadInfo {
                name: "Test Pad".to_string(), // TODO: Save the name in the script if it turns out to be relevant
            }),
        }));
    }

    *done = true;
}

#[allow(clippy::too_many_arguments)]
fn script_player(
    mut last_run: Local<Duration>,
    time: Res<Time<Real>>,
    script: Res<TestScript>,
    mut quit_events: EventWriter<StartAsserting>,
    mut kb_input: ResMut<Input<KeyCode>>,
    mut mouse_buttons: ResMut<Input<MouseButton>>,
    mut pad_buttons: ResMut<Input<GamepadButton>>,
    mut axis: ResMut<Axis<GamepadAxis>>,
    first_update: Option<Res<FirstUpdate>>,
) {
    let Some(start_time) = first_update else {
        return;
    };

    for ev in script
        .events
        .iter()
        .skip_while(|(event_time, _)| *event_time + start_time.0 <= *last_run)
        .take_while(|(event_time, _)| *event_time + start_time.0 <= time.elapsed())
        .map(|(_, input)| input)
    {
        match ev {
            UserInput::KeyPress(key) => kb_input.press(*key),
            UserInput::KeyRelese(key) => kb_input.release(*key),
            UserInput::MouseButtonPress(button) => mouse_buttons.press(*button),
            UserInput::MouseButtonRelease(button) => mouse_buttons.release(*button),
            UserInput::ControllerButtonPress(button) => pad_buttons.press(*button),
            UserInput::ControllerButtonRelease(button) => pad_buttons.release(*button),
            UserInput::ControllerAxisChange(key, value) => {
                axis.set(*key, *value);
            }
            UserInput::Quit => quit_events.send(StartAsserting),
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

fn run_asserts(
    mut commands: Commands,
    assert_sys: Res<AssertSystem>,
    start_events: EventReader<StartAsserting>,
    mut result_writer: EventWriter<TestQuitEvent>,
    time: Res<Time<Real>>,
    asserter: Res<Asserter>,
    mut started: Local<Option<Timer>>,
) {
    if let Some(ref mut start_time) = *started {
        if asserter.passed {
            result_writer.send(TestQuitEvent(true));
            *started = None;
        } else if start_time.tick(time.delta()).just_finished() {
            result_writer.send(TestQuitEvent(false));
            *started = None;
        } else {
            commands.run_system(assert_sys.0);
        }
    } else if !start_events.is_empty() {
        *started = Some(Timer::new(Duration::from_secs(5), TimerMode::Once));
    }
}

#[allow(clippy::too_many_arguments)]
fn delayed_exit(
    mut quit_events: ResMut<Events<AppExit>>,
    mut commands: Commands,
    callback: Res<QuitCallback>,
    mut custom_quit_events: EventReader<TestQuitEvent>,
    mut passed: Local<Option<bool>>,
    screenshot: Option<Res<EndScreenshot>>,
) {
    if passed.is_some() {
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

        if done {
            if passed.unwrap() {
                println!("Test passed");
                quit_events.send(AppExit);
            } else {
                // TODO: figure out a nicer way to fail the test
                panic!("Test failed");
            }
        }
    } else if !custom_quit_events.is_empty() {
        commands.run_system(callback.0);
        *passed = Some(custom_quit_events.read().next().unwrap().0);
    }
}
