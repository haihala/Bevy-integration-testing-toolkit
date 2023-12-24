use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, jump);
    }
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Jump,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(KeyCode::Space, Action::Jump)]),
        },
        Player,
        RigidBody::Dynamic,
        Velocity::default(),
        Sleeping::disabled(),
        Ccd::enabled(),
        Collider::ball(50.0),
        TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)),
    ));
}

// Query for the `ActionState` component in your game logic systems!
fn jump(mut commands: Commands, query: Query<(Entity, &ActionState<Action>), With<Player>>) {
    let (entity, action_state) = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(Action::Jump) {
        commands.entity(entity).insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 100.0),
            ..Default::default()
        });
    }
}
