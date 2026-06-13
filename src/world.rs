use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn generate_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TiledMap(asset_server.load("tilemap.tmx")));
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
