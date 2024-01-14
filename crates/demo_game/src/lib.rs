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
    pub insert_test_system: bool,
}

impl Plugin for DemoGamePlugin {
    fn build(&self, app: &mut App) {
        if self.show_inspector {
            app.add_plugins(WorldInspectorPlugin::new());
        }

        if self.insert_test_system {
            app.add_systems(Last, test_assert);
        }

        app.add_plugins((
            player::PlayerPlugin,
            world::PhysicsPlugin,
            assets::CustomAssetsPlugin,
            star::StarPlugin,
        ));
    }
}

fn test_assert(score: Query<&star::Points>, mut asserter: ResMut<Asserter>) {
    dbg!("test_assert");
    if score.single().0 == 2 {
        asserter.pass();
    }
}
