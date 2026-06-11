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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_plugins(PlayerStatePlugin)
            .add_systems(Startup, setup_player);
    }
}

pub fn setup_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn((
        (
            Player,
            Interactor,
            Speed(config.base_speed),
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
        (
            Idle,
            StateMachine::default()
                .trans::<Idle, _>(is_walking, Walking)
                .trans::<Idle, _>(is_running, Running)
                .trans::<Walking, _>(is_idle, Idle)
                .trans::<Walking, _>(is_running, Running)
                .trans::<Running, _>(is_idle, Idle)
                .trans::<Running, _>(is_walking, Walking),
            PlayerAction::default_input_map(),
            ActionState::<PlayerAction>::default(),
        ),
        (
            RigidBody::Dynamic,
            Collider::rectangle(config.size, config.size),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::default(),
            LinearDamping(5.0),
            CollisionEventsEnabled,
            CollisionLayers::new([GameLayer::Player], [GameLayer::Default, GameLayer::Interactable]),
        ),
    ));
}
