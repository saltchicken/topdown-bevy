use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::PlayerAction;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

pub fn on_enter(trigger: On<Add, Idle>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 1.0, 0.0);
    }
}

pub fn is_idle(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() == 0.0
}
