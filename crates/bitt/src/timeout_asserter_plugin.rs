use std::time::Duration;

use bevy::{app::AppExit, prelude::*};

use crate::TestWrangler;

#[derive(Debug, Resource)]
struct Timeout(Duration);

/// A plugin that will add an Asserter and panic if it runs for longer than the given duration.
///
/// Useful for cases when you want to test a combo of some systems in relative isolation.
#[derive(Debug)]
pub struct TimeoutAsserterPlugin(pub Duration);

impl Plugin for TimeoutAsserterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TestWrangler>()
            .insert_resource(Timeout(self.0))
            .add_systems(Update, (timeout, exit_on_asserter_result).chain());
    }
}

fn timeout(mut asserter: ResMut<TestWrangler>, time: Res<Time<Real>>, timeout: Res<Timeout>) {
    if time.elapsed() >= timeout.0 {
        dbg!("Test timed out");
        asserter.fail();
    }
}

fn exit_on_asserter_result(asserter: Res<TestWrangler>, mut exit: EventWriter<AppExit>) {
    if let Some(result) = asserter.outcome {
        if result {
            exit.send(AppExit);
        } else {
            panic!("Test failed");
        }
    }
}
