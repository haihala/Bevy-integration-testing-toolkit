use std::env;

use bitt::test_scenario_main;
use demo_game::DemoGamePlugin;

test_scenario_main!(
    "controller",
    DemoGamePlugin {
        show_inspector: false,
        insert_test_system: true,
    },
    env::var("CI").is_ok()
);
