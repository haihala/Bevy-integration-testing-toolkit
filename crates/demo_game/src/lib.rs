use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bitt::Asserter;

mod assets;
mod player;
mod star;
mod world;

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

pub fn test_assert(score: Query<&star::Points>, mut asserter: ResMut<Asserter>) {
    asserter.assert(score.single().0 == 1);
}
