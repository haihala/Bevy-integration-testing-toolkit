use bevy::prelude::*;

pub struct TestGearPlugin;

impl Plugin for TestGearPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(PreUpdate, test_system);
    }
}

fn setup() {
    // TODO
}

fn test_system() {
    // TODO
}
