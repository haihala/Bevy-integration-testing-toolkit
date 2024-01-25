use std::time::Duration;

use bevy::{app::AppExit, prelude::*};

/// A resource that can be used to state the outcome of this test.
#[derive(Resource, Debug, Default)]
pub struct Asserter {
    pub(crate) outcome: Option<bool>,
}

impl Asserter {
    /// Marks the current test as passed.
    /// Once a test is marked as failed or passed, it cannot be changed.
    pub fn pass(&mut self) {
        if self.outcome.is_none() {
            self.outcome = Some(true);
        }
    }

    /// Marks the current test as failed.
    /// Once a test is marked as failed or passed, it cannot be changed.
    pub fn fail(&mut self) {
        if self.outcome.is_none() {
            self.outcome = Some(false);
        }
    }
}

#[derive(Debug)]
pub struct AsserterPlugin;

impl Plugin for AsserterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Asserter>();
    }
}

#[derive(Debug, Resource)]
struct Timeout(Duration);

/// A plugin that will add an Asserter and panic if it runs for longer than the given duration.
///
/// Useful for cases when you want to test a combo of some systems in relative isolation.
#[derive(Debug)]
pub struct TimeoutAsserterPlugin(pub Duration);

impl Plugin for TimeoutAsserterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Asserter>()
            .insert_resource(Timeout(self.0))
            .add_systems(Update, (timeout, exit_on_asserter_result).chain());
    }
}

fn timeout(mut asserter: ResMut<Asserter>, time: Res<Time<Real>>, timeout: Res<Timeout>) {
    if time.elapsed() >= timeout.0 {
        dbg!("Test timed out");
        asserter.fail();
    }
}

fn exit_on_asserter_result(asserter: Res<Asserter>, mut exit: EventWriter<AppExit>) {
    if let Some(result) = asserter.outcome {
        if result {
            exit.send(AppExit);
        } else {
            panic!("Test failed");
        }
    }
}
