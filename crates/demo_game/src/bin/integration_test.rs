use std::env;

use bevy::prelude::*;

use bitt::{Asserter, PlaybackTestGear};

use demo_game::{DemoGamePlugin, Points};

fn main() {
    let script = env::var("BITT_SCRIPT").unwrap();

    App::new()
        .add_plugins((
            DefaultPlugins,
            PlaybackTestGear::new(script, env::var("CI").is_ok()),
            DemoGamePlugin {
                show_inspector: false,
            },
        ))
        .add_systems(Update, test_assert)
        .run();
}

fn test_assert(score: Query<&Points>, mut asserter: ResMut<Asserter>) {
    if score.single().0 == 2 {
        asserter.pass();
    }
}
