use bevy::prelude::*;
use bevy_integration_test_framework::*;
use demo_game::DemoGamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    if cfg!(feature = "integration_test") {
        app.add_plugins(PlaybackTestGear::new("plain_jumps".into()));
    }

    app.add_plugins(DemoGamePlugin).run();
}
