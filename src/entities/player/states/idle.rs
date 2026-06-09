use bevy::prelude::*;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

pub struct IdlePlugin;

impl Plugin for IdlePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_enter).add_observer(on_exit);
    }
}

fn on_enter(trigger: On<Add, Idle>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 1.0, 0.0);
    }
}

fn on_exit(_trigger: On<Remove, Idle>) {
    // No specific exit logic required
}
