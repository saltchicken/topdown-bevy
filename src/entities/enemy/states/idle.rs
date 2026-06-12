use bevy::prelude::*;
use crate::entities::enemy::{EnemyConfig, IdleTimer};

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

pub fn on_enter(
    trigger: On<Add, Idle>,
    mut query: Query<(&mut Sprite, &mut IdleTimer)>,
    config: Res<EnemyConfig>,
) {
    if let Ok((mut sprite, mut timer)) = query.get_mut(trigger.entity) {
        sprite.color = config.color_idle;
        timer.0.reset();
    }
}

pub fn on_update(time: Res<Time>, mut query: Query<&mut IdleTimer, With<Idle>>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}
