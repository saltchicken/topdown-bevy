use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::entities::player::{PlayerConfig, Speed};
use crate::input::PlayerAction;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Running;

pub fn on_enter(trigger: On<Add, Running>, mut query: Query<&mut Sprite>, config: Res<PlayerConfig>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = config.color_running;
    }
}

pub fn on_update(
    mut query: Query<(&mut LinearVelocity, &ActionState<PlayerAction>, &Speed), With<Running>>,
) {
    for (mut velocity, action_state, speed) in &mut query {
        let direction = action_state.axis_pair(&PlayerAction::Move);
        if direction.length_squared() > 0.0 {
            velocity.0 = direction.normalize() * speed.0;
        }
    }
}

pub fn is_running(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    crate::input::MovementIntention::evaluate(action_state) == crate::input::MovementIntention::Running
}
