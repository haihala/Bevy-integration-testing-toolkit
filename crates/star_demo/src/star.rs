use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::ReusedAssets, player::Player};

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

#[allow(clippy::type_complexity)]
fn detect_collisions(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<(&mut Transform, &Collider), With<Star>>,
        Query<&Transform, With<StarSpawnPoint>>,
    )>,
    reused_assets: Res<ReusedAssets>,
    mut points: Query<(&mut Text, &mut Points)>,
    rapier_context: Res<RapierContext>,
) {
    let players = queries.p0();
    let Ok(player_tf) = players.get_single() else {
        return;
    };
    let next_star_x = if player_tf.translation.x > 0.0 {
        -400.0
    } else {
        400.0
    };

    let mut stars = queries.p1();
    let Ok((ref mut star_tf, star_shape)) = stars.get_single_mut() else {
        return;
    };

    if rapier_context
        .intersection_with_shape(
            star_tf.translation.truncate(),
            0.0,
            star_shape,
            QueryFilter {
                flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
        )
        .is_some()
    {
        commands.spawn(AudioBundle {
            source: reused_assets.pling.clone(),
            ..default()
        });

        star_tf.translation.x = next_star_x;
        let (mut text, mut points) = points.single_mut();
        points.0 += 1;
        text.sections[0].value = points.0.to_string();
    };
}
