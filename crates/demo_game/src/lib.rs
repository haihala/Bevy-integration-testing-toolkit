use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod assets;
mod player;
mod star;
mod world;

pub use star::Points;

#[derive(Debug, Default)]
pub struct DemoGamePlugin {
    pub show_inspector: bool,
}

impl Plugin for DemoGamePlugin {
    fn build(&self, app: &mut App) {
        if self.show_inspector {
            app.add_plugins(WorldInspectorPlugin::new());
        }

        app.add_plugins((
            player::PlayerPlugin,
            world::PhysicsPlugin,
            assets::CustomAssetsPlugin,
            star::StarPlugin,
        ));
    }
}
