use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod assets;
mod player;
mod star;
mod world;

pub struct TestAppPlugin;

impl Plugin for TestAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldInspectorPlugin::new(),
            player::PlayerPlugin,
            world::PhysicsPlugin,
            assets::CustomAssetsPlugin,
            star::StarPlugin,
        ));
    }
}
