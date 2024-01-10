mod asserts;
pub use asserts::{AssertSystem, Asserter, AsserterPlugin};

mod input_playback;
pub use input_playback::PlaybackTestGear;

#[macro_export]
macro_rules! test_scenario {
    ($script_name:expr, $assert_system:path, $app_plugins:expr, $read_only:expr) => {
        use bevy::prelude::*;
        use bitt::{AssertSystem, Asserter, AsserterPlugin, PlaybackTestGear};

        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        app.add_plugins(PlaybackTestGear::new($script_name.into(), $read_only));
        app.add_plugins(AsserterPlugin);
        let assert_sys_id = app.world.register_system($assert_system);
        app.insert_resource(AssertSystem(assert_sys_id));

        app.add_plugins($app_plugins).run();
    };
}
