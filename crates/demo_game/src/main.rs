use bevy::prelude::*;
use demo_game::DemoGamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    if cfg!(feature = "integration_test") {
        app.add_plugins(bevy_integration_test_framework::TestGearPlugin);
    }

    app.add_plugins(DemoGamePlugin).run();
}
