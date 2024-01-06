mod asserts;
pub use asserts::{AssertSystem, Asserter, AsserterPlugin};

mod input_playback;
pub use input_playback::PlaybackTestGear;

#[macro_export]
macro_rules! test_scenario {
    ($script_name:expr, $assert_system:path, $appPlugins:path) => {
        use bevy::prelude::*;
        use bevy_integration_test_tool::{
            AssertSystem, Asserter, AsserterPlugin, PlaybackTestGear,
        };

        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        app.add_plugins(PlaybackTestGear::new($script_name.into()));
        app.add_plugins(AsserterPlugin);
        let assert_sys_id = app.world.register_system($assert_system);
        app.insert_resource(AssertSystem(assert_sys_id));

        app.add_plugins($appPlugins).run();
    };
}
