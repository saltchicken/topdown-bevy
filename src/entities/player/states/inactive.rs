use bevy::prelude::*;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Inactive;

pub struct InactivePlugin;

impl Plugin for InactivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_enter)
            .add_observer(on_exit);
    }
}

fn on_enter(trigger: On<Add, Inactive>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 1.0, 0.0);
    }
}

fn on_exit(_trigger: On<Remove, Inactive>) {
    // No specific exit logic required
}
