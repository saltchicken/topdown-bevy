use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::input::PlayerAction;
use crate::entities::player::Velocity;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Active;

pub struct ActivePlugin;

impl Plugin for ActivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_enter)
            .add_observer(on_exit)
            .add_systems(Update, on_update);
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
    mut query: Query<(&mut Velocity, &ActionState<PlayerAction>), With<Active>>,
) {
    for (mut velocity, action_state) in &mut query {
        let direction = action_state.axis_pair(&PlayerAction::Move);

        let speed = 300.0;

        velocity.0 = direction.normalize_or_zero() * speed;
    }
}
