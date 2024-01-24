use bevy::prelude::*;
use star_demo::DemoGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DemoGamePlugin {
            show_inspector: true,
        })
        .run();
}
