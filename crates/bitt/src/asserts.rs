use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Asserter {
    pub(crate) passed: bool,
}

impl Asserter {
    pub fn pass(&mut self) {
        self.passed = true;
    }
}

#[derive(Debug)]
pub struct AsserterPlugin;

impl Plugin for AsserterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Asserter>();
    }
}
