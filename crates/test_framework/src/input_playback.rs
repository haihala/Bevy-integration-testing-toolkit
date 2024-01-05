use std::{path::Path, time::Duration};

use serde::{Deserialize, Serialize};

use bevy::{
    app::AppExit,
    prelude::*,
    window::{exit_on_all_closed, exit_on_primary_closed},
};

use crate::AssertSystem;

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
        if let Some(script) = load_script(self.path.clone()) {
            app.insert_resource(script)
                .add_systems(First, script_player)
        } else {
            app.insert_resource(TestScript::default())
                .add_systems(First, script_recorder)
        }
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Resource)]
struct TestScript(Vec<(Duration, UserInput)>);

fn load_script(path: String) -> Option<TestScript> {
    let full_path = Path::new("test_framework").join(format!("{}.bitt_script", path));

    if full_path.exists() {
        let file = std::fs::read_to_string(full_path).unwrap();
        let script: TestScript = serde_json::from_str(&file).unwrap();
        Some(script)
    } else {
        None
    }
}

fn script_recorder() {
    // TODO: This ought to write input events to the script.
    dbg!("recorder");
}
fn script_player() {
    // TODO: This ougth to discard real inputs or at least override them with the script
    // Once script runs out, it should send the signal to quit the app
    dbg!("player");
}
fn run_asserts(mut commands: Commands, assert_sys: Res<AssertSystem>) {
    dbg!("framework");
    commands.run_system(assert_sys.0);
}
