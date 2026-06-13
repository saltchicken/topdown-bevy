use crate::entities::enemy::{EnemyBundle, EnemyConfig};
use crate::entities::interactables::chest::{ChestBundle, ChestConfig};
use crate::entities::interactables::coin::{CoinBundle, CoinConfig};
use crate::entities::player::{PlayerBundle, PlayerConfig};
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn generate_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TiledMap(asset_server.load("tilemap.tmx")));
}

pub fn spawn_tiled_entities(
    trigger: On<Add, TiledObject>,
    mut commands: Commands,
    query: Query<&TiledName>,
    chest_config: Res<ChestConfig>,
    coin_config: Res<CoinConfig>,
    player_config: Res<PlayerConfig>,
    enemy_config: Res<EnemyConfig>,
) {
    let entity = trigger.entity;

    // We fetch the TiledName associated with the entity that just received a TiledObject component
    if let Ok(tiled_name) = query.get(entity) {
        match tiled_name.0.as_str() {
            "Player" => {
                commands
                    .entity(entity)
                    .insert(PlayerBundle::new(&player_config));
            }
            "Enemy" => {
                commands
                    .entity(entity)
                    .insert(EnemyBundle::new(&enemy_config));
            }
            "Coin" => {
                commands
                    .entity(entity)
                    .insert(CoinBundle::new(1, &coin_config));
            }
            "Chest" => {
                commands
                    .entity(entity)
                    .insert(ChestBundle::new(50, &chest_config));
            }
            // Ignore objects that don't have a recognized class
            _ => {}
        }
    }
}

pub fn on_collider_created(trigger: On<TiledEvent<ColliderCreated>>, mut commands: Commands, query: Query<&TiledName>) {
    let event = trigger.event();

    if let Ok(name) = query.get(event.origin) {
        info!("Layer: {:?}", name.0.as_str());
    }

    if matches!(event.event.source, TiledColliderSource::TilesLayer) {
        commands.entity(event.origin).insert((
            RigidBody::Static,
            CollisionLayers::new([GameLayer::Default], [GameLayer::Player, GameLayer::Enemy]),
        ));
    }
}
