use std::{
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
    time::Duration,
};

use bevy::{
    app::AppExit,
    input::{
        gamepad::{
            GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnection,
            GamepadConnectionEvent, GamepadEvent, GamepadInfo,
        },
        mouse::{MouseMotion, MouseWheel},
        InputSystem,
    },
    prelude::*,
    render::view::screenshot::ScreenshotManager,
    utils::HashSet,
    window::PrimaryWindow,
};

use crate::{Asserter, PlaybackTestingOptions};

use super::{artefact_paths::ArtefactPaths, FirstUpdate, TestQuitEvent, TestScript, UserInput};

#[derive(Debug, Clone, Copy, Event)]
struct StartAsserting;

pub(crate) struct PlaybackPlugin {
    pub(crate) script: TestScript,
    pub(crate) artefact_path: PathBuf,
}

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        // This is a bit wonky, as it depends on the order the plugins get added
        let running_headless = app
            .world
            .query::<&PrimaryWindow>()
            .iter(&app.world)
            .next()
            .is_none();

        app.insert_resource(self.script.clone())
            .add_systems(
                PreUpdate,
                (connect_pads, script_player).chain().after(InputSystem),
            )
            .insert_resource(ArtefactPaths {
                base: self.artefact_path.clone(),
                running_headless,
            })
            .add_event::<StartAsserting>()
            .add_event::<TestQuitEvent>()
            .add_systems(
                Update,
                (
                    run_asserts,
                    create_artefact_dir.run_if(on_event::<StartAsserting>()),
                    pre_assert_screenshot.run_if(on_event::<StartAsserting>()),
                    post_assert_screenshot.run_if(on_event::<TestQuitEvent>()),
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
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    script: Res<TestScript>,
    mut quit_events: EventWriter<StartAsserting>,
    mut kb_input: ResMut<ButtonInput<KeyCode>>,
    mut mouse_buttons: ResMut<ButtonInput<MouseButton>>,
    mut pad_buttons: ResMut<ButtonInput<GamepadButton>>,
    mut axis: ResMut<Axis<GamepadAxis>>,
    mut mouse_scroll: EventWriter<MouseWheel>,
    mut mouse_movements: EventWriter<MouseMotion>,
    first_update: Option<Res<FirstUpdate>>,
    mut gamepad_event_writer: EventWriter<GamepadEvent>,
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
            UserInput::ControllerButtonPress(button) => {
                pad_buttons.press(*button);
                gamepad_event_writer.send(GamepadEvent::Button(GamepadButtonChangedEvent {
                    value: 1.0,
                    button_type: button.button_type,
                    gamepad: button.gamepad,
                }));
            }
            UserInput::ControllerButtonRelease(button) => {
                pad_buttons.release(*button);
                gamepad_event_writer.send(GamepadEvent::Button(GamepadButtonChangedEvent {
                    value: 0.0,
                    button_type: button.button_type,
                    gamepad: button.gamepad,
                }));
            }
            UserInput::ControllerAxisChange(key, value) => {
                axis.set(*key, *value);
                gamepad_event_writer.send(GamepadEvent::Axis(GamepadAxisChangedEvent {
                    gamepad: key.gamepad,
                    value: *value,
                    axis_type: key.axis_type,
                }));
            }
            UserInput::MouseScroll(scroll) => {
                mouse_scroll.send(*scroll);
            }
            UserInput::MouseMove(delta, position) => {
                mouse_movements.send(MouseMotion { delta: *delta });
                if let Ok(ref mut window) = window_query.get_single_mut() {
                    window.set_cursor_position(*position);
                }
            }
            UserInput::Quit => {
                quit_events.send(StartAsserting);
            }
        }
    }

    *last_run = time.elapsed();
}

fn create_artefact_dir(path: Res<ArtefactPaths>, mut has_ran: Local<bool>) {
    if *has_ran {
        return;
    }

    if path.base.exists() {
        remove_dir_all(path.base.clone()).unwrap();
    }

    create_dir_all(path.base.clone()).unwrap();

    *has_ran = true;
}

fn pre_assert_screenshot(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    path: Res<ArtefactPaths>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    if let Ok(win) = main_window.get_single() {
        screenshot_manager
            .save_screenshot_to_disk(win, path.pre_assert_screenshot())
            .unwrap();
    }

    *has_ran = true;
}

fn post_assert_screenshot(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    path: Res<ArtefactPaths>,
    mut has_ran: Local<bool>,
) {
    if *has_ran {
        return;
    }

    if let Ok(win) = main_window.get_single() {
        screenshot_manager
            .save_screenshot_to_disk(win, path.post_assert_screenshot())
            .unwrap();
    }

    *has_ran = true;
}

fn run_asserts(
    mut start_events: EventReader<StartAsserting>,
    mut result_writer: EventWriter<TestQuitEvent>,
    time: Res<Time<Real>>,
    asserter: Res<Asserter>,
    options: Res<PlaybackTestingOptions>,
    mut started: Local<Option<Timer>>,
) {
    if let Some(ref mut start_time) = *started {
        if asserter.outcome == Some(true) {
            result_writer.send(TestQuitEvent(true));
            *started = None;
        } else if asserter.outcome == Some(false) || start_time.tick(time.delta()).just_finished() {
            result_writer.send(TestQuitEvent(false));
            *started = None;
        }
    } else if start_events.read().next().is_some() {
        *started = Some(Timer::from_seconds(options.assert_window, TimerMode::Once));
    }
}

#[allow(clippy::too_many_arguments)]
fn delayed_exit(
    mut quit_events: ResMut<Events<AppExit>>,
    mut custom_quit_events: EventReader<TestQuitEvent>,
    mut result: Local<Option<bool>>,
    artefacts: Res<ArtefactPaths>,
) {
    if let Some(passed) = *result {
        if artefacts.saved() {
            if passed {
                println!("Test passed");
                quit_events.send(AppExit);
            } else {
                // TODO: figure out a nicer way to fail the test
                panic!("Test failed");
            }
        }
    } else if !custom_quit_events.is_empty() {
        *result = Some(custom_quit_events.read().next().unwrap().0);
    }
}
