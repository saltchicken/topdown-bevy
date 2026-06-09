use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use super::components::{Player, Speed};
use super::states::{idle::Idle, walking::Walking};
use crate::input::PlayerAction;

pub fn is_moving(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else { return false; };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
}

pub fn is_idle(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else { return false; };
    action_state.axis_pair(&PlayerAction::Move).length_squared() == 0.0
}

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Speed(300.0),
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Idle,
        StateMachine::default()
            .trans::<Idle, _>(is_moving, Walking)
            .trans::<Walking, _>(is_idle, Idle),
        PlayerAction::default_input_map(),
        ActionState::<PlayerAction>::default(),
        RigidBody::Dynamic,
        Collider::rectangle(40.0, 40.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        LinearDamping(5.0),
    ));
}
