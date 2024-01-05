use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod assets;
mod player;
mod star;
mod world;

pub struct DemoGamePlugin;

impl Plugin for DemoGamePlugin {
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
