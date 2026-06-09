use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Inactive;

pub struct InactivePlugin;

impl Plugin for InactivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_enter)
            .add_observer(on_exit)
            .add_systems(Update, on_update);
    }
}

fn on_enter(trigger: On<Add, Inactive>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 1.0, 0.0);
    }
}

fn on_update(mut query: Query<&mut LinearVelocity, With<Inactive>>) {
    for mut velocity in &mut query {
        velocity.0 = Vec2::ZERO;
    }
}

fn on_exit(_trigger: On<Remove, Inactive>) {
    // No specific exit logic required
}
