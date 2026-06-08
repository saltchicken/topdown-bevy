use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::PlayerAction;

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
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Active>>,
) {
    for (mut transform, action_state) in &mut query {
        let mut direction = Vec2::ZERO;
        if action_state.pressed(&PlayerAction::Up) {
            direction.y += 1.0;
        }
        if action_state.pressed(&PlayerAction::Down) {
            direction.y -= 1.0;
        }
        if action_state.pressed(&PlayerAction::Left) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&PlayerAction::Right) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            transform.translation += direction.normalize().extend(0.0) * 300.0 * time.delta_secs();
        }
    }
}
