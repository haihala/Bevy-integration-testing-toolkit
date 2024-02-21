use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

pub struct ClickDemoPlugin;

#[derive(Debug, Resource)]
pub struct Points(pub u32);

#[derive(Debug, Component)]
pub struct Clickable;
#[derive(Debug, Component)]
pub struct TimeText;
#[derive(Debug, Component)]
pub struct ScoreText;

impl Plugin for ClickDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (register_clicks, update_score, move_target, handle_timer).chain(),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Points(0));

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        Clickable,
    ));
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|cb| {
            cb.spawn((
                TextBundle {
                    text: Text::from_sections(vec![
                        TextSection {
                            value: "Score: ".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                    ]),
                    ..default()
                },
                ScoreText,
            ));
            cb.spawn((
                TextBundle {
                    text: Text::from_sections(vec![
                        TextSection {
                            value: "Time: ".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "60".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        },
                    ]),
                    ..default()
                },
                TimeText,
            ));
        });
}

fn register_clicks(
    buttons: Res<ButtonInput<MouseButton>>,
    mut points: ResMut<Points>,
    clickable_query: Query<&Transform, With<Clickable>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(clickable_tf) = clickable_query.get_single() else {
        // This happens when the game is over.
        return;
    };

    let (cam, cam_tf) = camera_query.single();
    let window = window_query.single();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_tf, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if (world_pos - clickable_tf.translation.truncate()).length() < 50.0 {
            points.0 += 1;
        } else if points.0 > 0 {
            points.0 -= 1;
        }
    }
}

fn update_score(points: Res<Points>, mut score_text_query: Query<&mut Text, With<ScoreText>>) {
    if points.is_changed() {
        score_text_query.single_mut().sections[1].value = points.0.to_string();
    }
}

fn move_target(points: Res<Points>, mut clickable_query: Query<&mut Transform, With<Clickable>>) {
    if points.is_changed() {
        let angle = points.0 as f32;
        clickable_query.single_mut().translation = 300.0 * Vec3::new(angle.cos(), angle.sin(), 0.0);
    }
}

fn handle_timer(
    mut time_text_query: Query<&mut Text, With<TimeText>>,
    time: Res<Time>,
    mut commands: Commands,
    clickable_query: Query<Entity, With<Clickable>>,
) {
    time_text_query.single_mut().sections[1].value = if time.elapsed_seconds() > 60.0 {
        for entity in clickable_query.iter() {
            // This prevents further clicks upon time over
            commands.entity(entity).remove::<Clickable>();
        }

        "Over!".to_string()
    } else {
        (60.0 - time.elapsed_seconds()).ceil().to_string()
    };
}
