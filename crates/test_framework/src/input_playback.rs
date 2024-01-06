use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use bevy::{
    app::AppExit,
    prelude::*,
    window::{exit_on_all_closed, exit_on_primary_closed},
};

use crate::AssertSystem;

#[derive(Debug, Resource)]
struct ScriptPath(PathBuf);

#[derive(Debug)]
pub struct PlaybackTestGear {
    path: String,
}

impl PlaybackTestGear {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Plugin for PlaybackTestGear {
    fn build(&self, app: &mut App) {
        let (maybe_script, path) = load_script(self.path.clone());

        if let Some(script) = maybe_script {
            app.insert_resource(script)
                .add_systems(First, script_player)
        } else {
            app.insert_resource(TestScript::default())
                .add_systems(First, script_recorder)
                .add_systems(
                    PostUpdate,
                    save_script.run_if(on_event::<AppExit>()).after(run_asserts),
                )
        }
        .insert_resource(ScriptPath(path))
        .add_systems(
            PostUpdate,
            run_asserts
                .run_if(on_event::<AppExit>())
                .after(exit_on_primary_closed)
                .after(exit_on_all_closed),
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UserInput {
    KeyPress(KeyCode),
    KeyRelese(KeyCode),
    Quit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resource)]
struct TestScript(Vec<(Duration, UserInput)>);

fn load_script(path: String) -> (Option<TestScript>, PathBuf) {
    let full_path = Path::new("test_scripts").join(format!("{}.bitt_script", path));

    (
        if full_path.exists() {
            let file = std::fs::read_to_string(&full_path).unwrap();
            let script: TestScript = serde_json::from_str(&file).unwrap();
            Some(script)
        } else {
            None
        },
        full_path,
    )
}

fn script_recorder(
    mut script: ResMut<TestScript>,
    time: Res<Time<Real>>,
    input: Res<Input<KeyCode>>,
) {
    for key in input.get_just_pressed() {
        script.0.push((time.elapsed(), UserInput::KeyPress(*key)));
    }

    for key in input.get_just_released() {
        script.0.push((time.elapsed(), UserInput::KeyRelese(*key)));
    }
}

fn save_script(script: Res<TestScript>, path: Res<ScriptPath>, time: Res<Time<Real>>) {
    let mut script = script.clone();
    script.0.push((time.elapsed(), UserInput::Quit));

    let prefix = path.0.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let file = std::fs::File::create(&path.0).unwrap();
    serde_json::to_writer(file, &script).unwrap();
}

fn script_player(
    mut last_run: Local<Duration>,
    time: Res<Time<Real>>,
    script: Res<TestScript>,
    mut kb_input: ResMut<Input<KeyCode>>,
    mut quit_events: ResMut<Events<AppExit>>,
) {
    for ev in script
        .0
        .iter()
        .skip_while(|(event_time, _)| *event_time < *last_run)
        .take_while(|(event_time, _)| *event_time < time.elapsed())
        .map(|(_, input)| input)
    {
        match ev {
            UserInput::KeyPress(key) => kb_input.press(*key),
            UserInput::KeyRelese(key) => kb_input.release(*key),
            UserInput::Quit => quit_events.send(AppExit),
        }
    }

    *last_run = time.elapsed();
}
fn run_asserts(mut commands: Commands, assert_sys: Res<AssertSystem>) {
    commands.run_system(assert_sys.0);
}
