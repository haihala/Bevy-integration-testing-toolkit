use std::env;

use bevy::prelude::*;

use bitt::{Asserter, PlaybackTestGear, PlaybackTestingOptions};

use star_demo::{DemoGamePlugin, Points};

fn main() {
    let script = env::var("BITT_SCRIPT").unwrap();

    App::new()
        .add_plugins((
            DefaultPlugins,
            PlaybackTestGear::new(
                script,
                PlaybackTestingOptions {
                    read_only: env::var("CI").is_ok(),
                    ..default()
                },
            ),
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
