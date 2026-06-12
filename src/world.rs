use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use crate::entities::enemy::{EnemyBundle, EnemyConfig};
use crate::entities::interactables::coin::{CoinBundle, CoinConfig};
use crate::entities::player::{PlayerBundle, PlayerConfig};

pub fn generate_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(TiledMap(asset_server.load("tilemap.tmx")));
}

pub fn spawn_tiled_entities(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &TiledName), Added<TiledObject>>,
    coin_config: Res<CoinConfig>,
    player_config: Res<PlayerConfig>,
    enemy_config: Res<EnemyConfig>,
) {
    for (entity, transform, tiled_name) in &query {
        let position = transform.translation.truncate();
        
        match tiled_name.0.as_str() { 
            "Player" => {
                commands
                    .entity(entity)
                    .insert(PlayerBundle::new(&player_config, position));
            }
            "Enemy" => {
                commands
                    .entity(entity)
                    .insert(EnemyBundle::new(&enemy_config, position));
            }
            "Coin" => {
                commands
                    .entity(entity)
                    .insert(CoinBundle::new(1, position, &coin_config));
            }
            // Ignore objects that don't have a recognized class
            _ => {} 
        }
    }
}
