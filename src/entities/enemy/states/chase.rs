use avian2d::prelude::*;
use bevy::prelude::*;
use crate::entities::enemy::EnemyConfig;
use crate::entities::player::Player;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Chase;

pub fn on_enter(trigger: On<Add, Chase>, mut query: Query<&mut Sprite>, config: Res<EnemyConfig>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = config.color_chase;
    }
}

pub fn on_update(
    mut enemy_query: Query<(&mut LinearVelocity, &Transform), With<Chase>>,
    player_query: Query<&Transform, With<Player>>,
    config: Res<EnemyConfig>,
) {
    let Ok(player_transform) = player_query.single() else { return; };
    for (mut velocity, enemy_transform) in &mut enemy_query {
        let direction = (player_transform.translation.truncate() - enemy_transform.translation.truncate()).normalize_or_zero();
        velocity.0 = direction * config.chase_speed;
    }
}
