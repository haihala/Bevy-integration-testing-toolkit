use bevy::prelude::*;
use bevy_integration_test_tool::*;
use demo_game::DemoGamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    if cfg!(feature = "integration_test") {
        app.add_plugins(PlaybackTestGear::new("plain_jumps".into()));
        let assert_sys_id = app.world.register_system(demo_game::test_assert);
        app.insert_resource(AssertSystem(assert_sys_id));
    }

    app.add_plugins(DemoGamePlugin).run();
}
