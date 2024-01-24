use std::env;

use bevy::prelude::*;

use bitt::{Asserter, HeadlessDefaultPlugins, PlaybackTestGear, PlaybackTestingOptions};

use star_demo::{DemoGamePlugin, Points};

fn main() {
    let script = env::var("BITT_SCRIPT").unwrap();

    let mut app = App::new();

    if env::var("HEADLESS").is_ok() {
        app.add_plugins(HeadlessDefaultPlugins);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.add_plugins((
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
