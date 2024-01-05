use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::assets::ReusedAssets;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_star)
            .add_systems(Update, (spin_star, detect_collisions));
    }
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Points(pub usize);

#[derive(Component)]
struct Star;

#[derive(Component)]
struct StarSpawnPoint;

fn spawn_star(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Star,
        Name::new("Star"),
        Collider::ball(40.0),
        Sensor,
        SpriteBundle {
            texture: asset_server.load("star.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 400.0, 0.0),
            ..default()
        },
    ));

    commands
        .spawn((SpatialBundle::default(), Name::new("Spawn points")))
        .with_children(|root| {
            for i in 0..10 {
                root.spawn((
                    StarSpawnPoint,
                    Name::new("Spawn point"),
                    SpatialBundle::from(Transform::from_xyz((i - 5) as f32 * 100.0, 400.0, 0.0)),
                ));
            }
        });

    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "0",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            },
        )]),
        Points(0),
    ));
}

fn spin_star(time: Res<Time>, mut query: Query<&mut Transform, With<Star>>) {
    for mut transform in &mut query {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 0.5));
    }
}

// Likely due to system ordering, sound of star pickup is played twice.
// This is a workaround to prevent that.
// Not happy with it, but I'm prioritizing.
#[derive(Component)]
struct RespawnCooldown;

fn detect_collisions(
    mut commands: Commands,
    player_query: Query<&KinematicCharacterControllerOutput>,
    reused_assets: Res<ReusedAssets>,
    mut stars: Query<&mut Transform, (With<Star>, Without<StarSpawnPoint>)>,
    spawn_points: Query<&mut Transform, (With<StarSpawnPoint>, Without<Star>)>,
    cooldown: Query<&RespawnCooldown>,
    mut points: Query<(&mut Text, &mut Points)>,
) {
    if !cooldown.is_empty() {
        return;
    }

    let Ok(cc) = player_query.get_single() else {
        return;
    };

    for contact in &cc.collisions {
        let Ok(ref mut star_tf) = stars.get_mut(contact.entity) else {
            continue;
        };

        commands.spawn((
            AudioBundle {
                source: reused_assets.pling.clone(),
                ..default()
            },
            RespawnCooldown,
        ));

        let furtherst_spawn_point = spawn_points
            .iter()
            .max_by_key(|sp| {
                (sp.translation.truncate() - contact.character_translation).length() as usize
            })
            .unwrap();

        star_tf.translation = furtherst_spawn_point.translation;
        let (mut text, mut points) = points.single_mut();
        points.0 += 1;
        text.sections[0].value = points.0.to_string();
        return;
    }
}
