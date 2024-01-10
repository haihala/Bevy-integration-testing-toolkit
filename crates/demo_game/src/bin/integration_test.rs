use std::env;

use bitt::test_scenario;
use demo_game::{test_assert, DemoGamePlugin};

fn main() {
    test_scenario!(
        "plain_jumps",
        test_assert,
        DemoGamePlugin {
            show_inspector: false,
        },
        env::var("CI").is_ok()
    );
}
