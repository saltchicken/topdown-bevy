use bevy::prelude::*;
use crate::entities::enemy::{EnemyBundle, EnemyConfig};
use crate::entities::interactables::coin::{CoinBundle, CoinConfig};
use crate::entities::player::{PlayerBundle, PlayerConfig};

pub fn generate_level(
    mut commands: Commands,
    coin_config: Res<CoinConfig>,
    player_config: Res<PlayerConfig>,
    enemy_config: Res<EnemyConfig>,
) {
    // Spawn the player
    commands.spawn(PlayerBundle::new(&player_config, Vec2::ZERO));

    // Spawn a high-value coin
    commands.spawn(CoinBundle::new(5, Vec2::new(150.0, 100.0), &coin_config));

    // Spawn a regular coin
    commands.spawn(CoinBundle::new(1, Vec2::new(-50.0, 20.0), &coin_config));

    // Spawn an enemy
    commands.spawn(EnemyBundle::new(&enemy_config, Vec2::new(400.0, -200.0)));
}
