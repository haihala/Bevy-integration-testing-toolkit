use bevy::prelude::*;

/// A resource that can be used to state the outcome of this test.
#[derive(Resource, Debug, Default)]
pub struct TestWrangler {
    pub(crate) outcome: Option<bool>,
    pub(crate) started: bool,
}

impl TestWrangler {
    /// Starts recording or playback depending on mode.
    /// **IMPORTANT**: Will not work unless you set `auto_start` to `false` in `PlaybackTestingOptions`
    /// This is idempotent, calling it multiple times does nothing.
    pub fn start(&mut self) {
        self.started = true;
    }

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
