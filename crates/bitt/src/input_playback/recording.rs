use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use bevy::{
    app::AppExit,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    utils::HashMap,
};

use crate::Asserter;

use super::{FirstUpdate, TestScript, UserInput};

#[derive(Debug, Clone, Copy, Event)]
struct SaveQuitEvent;

#[derive(Debug, Clone, Resource)]
struct ScriptPath(PathBuf);

pub(crate) struct RecordingPlugin {
    pub(crate) script_path: PathBuf,
}

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestScript::default())
            .add_systems(First, (script_recorder, recording_asserter).chain())
            .add_event::<SaveQuitEvent>()
            .insert_resource(ScriptPath(self.script_path.clone()))
            .add_systems(PostUpdate, save_script.run_if(on_event::<SaveQuitEvent>()));
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
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<MouseMotion>,
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

    for scroll in scroll_evr.read() {
        script
            .events
            .push((timestamp, UserInput::MouseScroll(*scroll)));
    }

    for motion in motion_evr.read() {
        script
            .events
            .push((timestamp, UserInput::MouseMove(motion.delta)));
    }
}

fn recording_asserter(
    asserter: ResMut<Asserter>,
    mut quit_events: EventWriter<SaveQuitEvent>,
    mut delay: Local<Option<Timer>>,
    time: Res<Time<Real>>,
) {
    if let Some(ref mut timer) = *delay {
        if timer.tick(time.delta()).just_finished() {
            quit_events.send(SaveQuitEvent);
        }
    } else if asserter.passed {
        *delay = Some(Timer::from_seconds(0.2, TimerMode::Once));
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
