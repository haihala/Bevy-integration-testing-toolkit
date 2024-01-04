use bevy::prelude::*;

pub struct CustomAssetsPlugin;

impl Plugin for CustomAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_reused_assets)
            .add_systems(Update, despawn_finished_audio_sinks);
    }
}

#[derive(Resource)]
pub struct ReusedAssets {
    pub hop: Handle<AudioSource>,
    pub pling: Handle<AudioSource>,
}

fn load_reused_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ReusedAssets {
        hop: asset_server.load("hop.ogg"),
        pling: asset_server.load("pling.ogg"),
    });
}

fn despawn_finished_audio_sinks(mut commands: Commands, targets: Query<(Entity, &AudioSink)>) {
    for (entity, sink) in &targets {
        if sink.empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
