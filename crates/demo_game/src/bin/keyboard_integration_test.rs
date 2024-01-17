use std::env;

use bevy::prelude::*;

use bitt::{AsserterPlugin, PlaybackTestGear};

use demo_game::DemoGamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlaybackTestGear::new("keyboard".into(), env::var("CI").is_ok()),
            AsserterPlugin,
        ))
        .add_plugins(DemoGamePlugin {
            show_inspector: false,
            insert_test_system: true,
        })
        .run();
}
