use std::env;

use bitt::test_scenario;
use demo_game::{test_assert, DemoGamePlugin};

test_scenario!(
    "keyboard",
    test_assert,
    DemoGamePlugin {
        show_inspector: false,
    },
    env::var("CI").is_ok()
);
