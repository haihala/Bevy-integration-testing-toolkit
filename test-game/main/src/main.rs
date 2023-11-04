use bevy::prelude::*;
use lib::TestAppPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TestAppPlugin)
        .run();
}
