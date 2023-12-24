use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod physics;
mod player;

pub struct TestAppPlugin;

impl Plugin for TestAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldInspectorPlugin::new(),
            player::PlayerPlugin,
            physics::PhysicsPlugin,
        ));
    }
}
