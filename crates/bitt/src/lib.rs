mod asserts;
pub use asserts::{Asserter, AsserterPlugin};

mod input_playback;
pub use input_playback::PlaybackTestGear;

#[macro_export]
macro_rules! test_scenario_main {
    ($script_name:expr, $app_plugins:expr, $read_only:expr) => {
        fn main() {
            use bevy::prelude::*;
            use bitt::{Asserter, AsserterPlugin, PlaybackTestGear};

            App::new()
                .add_plugins((
                    DefaultPlugins,
                    PlaybackTestGear::new($script_name.into(), $read_only),
                    AsserterPlugin,
                ))
                .add_plugins($app_plugins)
                .run();
        }
    };
}
