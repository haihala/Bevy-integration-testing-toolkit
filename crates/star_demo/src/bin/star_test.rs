use std::env;

use bevy::prelude::*;

use bitt::{HeadlessDefaultPlugins, PlaybackTestGear, PlaybackTestingOptions, TestWrangler};

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
                manual_start: true,
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

fn test_assert(score: Query<&Points>, mut wrangler: ResMut<TestWrangler>) {
    wrangler.start();

    if score.single().0 == 2 {
        wrangler.pass();
    }
}
