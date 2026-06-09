use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::input::PlayerAction;
use crate::entities::player::components::Speed;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Active;

pub struct ActivePlugin;

impl Plugin for ActivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_enter)
            .add_observer(on_exit)
            .add_systems(FixedUpdate, on_update);
    }
}

fn on_enter(trigger: On<Add, Active>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(1.0, 0.0, 0.0);
    }
}

fn on_exit(_trigger: On<Remove, Active>) {
    // No specific exit logic required
}

fn on_update(
    mut query: Query<(&mut LinearVelocity, &ActionState<PlayerAction>, &Speed), With<Active>>,
) {
    for (mut velocity, action_state, speed) in &mut query {
        let direction = action_state.axis_pair(&PlayerAction::Move);
        velocity.0 = direction.normalize_or_zero() * speed.0;
    }
}
