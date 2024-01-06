use std::env;

use bevy_integration_test_tool::test_scenario;
use demo_game::{test_assert, DemoGamePlugin};

fn main() {
    test_scenario!(
        "plain_jumps",
        test_assert,
        DemoGamePlugin,
        env::var("CI").is_ok()
    );
}
