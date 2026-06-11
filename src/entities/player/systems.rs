use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use super::components::{Player, Speed};
use super::config::PlayerConfig;
use super::states::{idle::Idle, running::Running, walking::Walking};
use crate::input::PlayerAction;

use crate::entities::interactables::components::Interactor;
use crate::physics::GameLayer;

pub fn is_walking(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
        && !action_state.pressed(&PlayerAction::Run)
}

pub fn is_running(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
        && action_state.pressed(&PlayerAction::Run)
}

pub fn is_idle(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() == 0.0
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
