use bevy::prelude::*;
use demo_game::DemoGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DemoGamePlugin {
            show_inspector: true,
        })
        .run();
}
