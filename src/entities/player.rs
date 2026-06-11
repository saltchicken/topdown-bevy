pub mod states;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::entities::interactables::Interactor;
use crate::input::PlayerAction;
use crate::physics::GameLayer;

use self::states::idle::*;
use self::states::running::*;
use self::states::walking::*;
use self::states::PlayerStatePlugin;

#[derive(Component, Default, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Speed(pub f32);

#[derive(Resource)]
pub struct PlayerConfig {
    pub size: f32,
    pub base_speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            size: 40.0,
            base_speed: 300.0,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub interactor: Interactor,
    pub speed: Speed,
    pub sprite: Sprite,
    pub transform: Transform,
    pub idle: Idle,
    pub state_machine: StateMachine,
    pub input_map: InputMap<PlayerAction>,
    pub action_state: ActionState<PlayerAction>,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub linear_velocity: LinearVelocity,
    pub linear_damping: LinearDamping,
    pub collision_events: CollisionEventsEnabled,
    pub collision_layers: CollisionLayers,
}

impl PlayerBundle {
    pub fn new(config: &PlayerConfig, position: Vec2) -> Self {
        Self {
            player: Player,
            interactor: Interactor,
            speed: Speed(config.base_speed),
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            idle: Idle,
            state_machine: StateMachine::default()
                .trans::<Idle, _>(is_walking, Walking)
                .trans::<Idle, _>(is_running, Running)
                .trans::<Walking, _>(is_idle, Idle)
                .trans::<Walking, _>(is_running, Running)
                .trans::<Running, _>(is_idle, Idle)
                .trans::<Running, _>(is_walking, Walking),
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::<PlayerAction>::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(config.size, config.size),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            linear_velocity: LinearVelocity::default(),
            linear_damping: LinearDamping(5.0),
            collision_events: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new([GameLayer::Player], [GameLayer::Default, GameLayer::Interactable]),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_plugins(PlayerStatePlugin);
    }
}
