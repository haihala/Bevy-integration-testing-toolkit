use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource, Debug, Default)]
pub struct Asserter {
    pub(crate) passed: bool,
    pub(crate) ran: bool,
}

impl Asserter {
    pub fn assert(&mut self, condition: bool) {
        self.passed |= condition;
        self.ran = true;
    }
}

#[derive(Resource, Debug)]
pub struct AssertSystem(pub SystemId);

#[derive(Debug)]
pub struct AsserterPlugin;

impl Plugin for AsserterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Asserter>();
    }
}
