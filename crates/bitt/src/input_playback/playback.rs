use std::{
    fs::{create_dir_all, remove_dir_all, File},
    path::PathBuf,
    time::Duration,
};

use bevy::{
    app::AppExit,
    input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent, GamepadInfo},
    prelude::*,
    render::view::screenshot::ScreenshotManager,
    utils::HashSet,
    window::PrimaryWindow,
};

use crate::{AssertSystem, Asserter};

use super::{FirstUpdate, TestScript, UserInput};

#[derive(Debug, Resource)]
struct ArtefactPath(PathBuf);

#[derive(Debug, Clone, Copy, Event)]
struct StartAsserting;
#[derive(Debug, Clone, Copy, Event)]
struct TestQuitEvent(bool);

pub(crate) struct PlaybackPlugin {
    pub(crate) script: TestScript,
    pub(crate) artefact_path: PathBuf,
}

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.script.clone())
            .add_systems(First, (connect_pads, script_player).chain())
            .insert_resource(ArtefactPath(self.artefact_path.clone()))
            .add_event::<StartAsserting>()
            .add_event::<TestQuitEvent>()
            .add_systems(
                Update,
                (
                    run_asserts,
                    record_artefacts.run_if(on_event::<TestQuitEvent>()),
                    delayed_exit,
                )
                    .chain(),
            );
    }
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
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    if path.0.exists() {
        remove_dir_all(path.0.clone()).unwrap();
    }
    create_dir_all(path.0.clone()).unwrap();
    let img_path = path.0.clone().join("screenshot.png");
    screenshot_manager
        .save_screenshot_to_disk(main_window.single(), img_path.clone())
        .unwrap();

    commands.insert_resource(EndScreenshot(img_path));

    *has_ran = true;
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
        *passed = Some(custom_quit_events.read().next().unwrap().0);
    }
}
