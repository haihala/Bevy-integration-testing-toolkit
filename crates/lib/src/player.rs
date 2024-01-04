use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_input);
    }
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Jump,
    Move,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(KeyCode::Space, Action::Jump)])
                .insert(
                    VirtualDPad {
                        up: KeyCode::Up.into(),
                        down: KeyCode::Down.into(),
                        left: KeyCode::Left.into(),
                        right: KeyCode::Right.into(),
                    },
                    Action::Move,
                )
                .build(),
        },
        Player,
        RigidBody::KinematicVelocityBased,
        KinematicCharacterController::default(),
        Collider::ball(50.0),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        },
    ));
}

fn player_input(
    mut query: Query<(&mut KinematicCharacterController, &ActionState<Action>), With<Player>>,
    mut jump_shift: Local<f32>,
) {
    let (mut character_controller, action_state) = query.single_mut();

    if action_state.just_pressed(Action::Jump) {
        *jump_shift = 20.0;
    }

    // Gravity
    *jump_shift -= 1.0;

    character_controller.translation = Some(Vec2::new(
        action_state.axis_pair(Action::Move).unwrap().x() * 10.0,
        *jump_shift,
    ));
}
