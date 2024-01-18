use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Asserter {
    pub(crate) outcome: Option<bool>,
}

impl Asserter {
    pub fn pass(&mut self) {
        if self.outcome.is_none() {
            self.outcome = Some(true);
        }
    }

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
