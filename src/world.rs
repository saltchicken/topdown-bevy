use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use crate::entities::enemy::{EnemyBundle, EnemyConfig};
use crate::entities::interactables::coin::{CoinBundle, CoinConfig};
use crate::entities::player::{PlayerBundle, PlayerConfig};
use crate::physics::GameLayer;

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

pub fn assign_terrain_collision_layers(
    mut commands: Commands,
    // Filter out entities we already configured manually to ensure we only hit map geometry
    query: Query<Entity, (Added<Collider>, Without<TiledObject>)>,
) {
    for entity in &query {
        info!("🛠️ DEBUG: Caught terrain collider on Entity {:?}", entity);
        commands.entity(entity).insert((
            RigidBody::Static,
            CollisionLayers::new(
                [GameLayer::Default],
                [GameLayer::Player, GameLayer::Enemy],
            ),
        ));
    }
}
