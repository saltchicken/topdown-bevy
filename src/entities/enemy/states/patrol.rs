use avian2d::prelude::*;
use bevy::prelude::*;
use crate::entities::enemy::{EnemyConfig, MoveDirection, PatrolTimer};

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Patrol;

pub fn on_enter(
    trigger: On<Add, Patrol>,
    mut query: Query<(&mut Sprite, &mut PatrolTimer)>,
    config: Res<EnemyConfig>,
) {
    if let Ok((mut sprite, mut timer)) = query.get_mut(trigger.entity) {
        sprite.color = config.color_patrol;
        timer.0.reset();
    }
}

pub fn on_update(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &mut PatrolTimer, &mut MoveDirection), With<Patrol>>,
    config: Res<EnemyConfig>,
) {
    for (mut velocity, mut timer, mut direction) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            direction.0 = -direction.0;
        }
        velocity.0 = direction.0 * config.patrol_speed;
    }
}
