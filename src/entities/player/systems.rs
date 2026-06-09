use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use super::components::{Player, Speed};
use super::states::{active::Active, inactive::Inactive};
use crate::input::PlayerAction;

pub fn toggle_pressed(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else { return false; };
    action_state.just_pressed(&PlayerAction::Toggle)
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
        Inactive,
        StateMachine::default()
            .trans::<Inactive, _>(toggle_pressed, Active)
            .trans::<Active, _>(toggle_pressed, Inactive),
        PlayerAction::default_input_map(),
        ActionState::<PlayerAction>::default(),
        RigidBody::Dynamic,
        Collider::rectangle(40.0, 40.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
    ));
}
