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
    query: Query<(Entity, &Transform, &Name), Added<Transform>>,
    coin_config: Res<CoinConfig>,
    player_config: Res<PlayerConfig>,
    enemy_config: Res<EnemyConfig>,
) {
    for (entity, transform, name) in &query {
        let position = transform.translation.truncate();
        let name_str = name.as_str();

        let parsed_name = name_str
            .split_once('(')
            .and_then(|(_, inner)| inner.strip_suffix(')'))
            .unwrap_or(name_str);

        // Match against the object's Name field from Tiled
        match parsed_name {
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
            _ => {} 
        }
    }
}
