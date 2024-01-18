use bevy::prelude::*;

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
