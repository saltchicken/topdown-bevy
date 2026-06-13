use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct EnvironmentLayer {
    pub is_sensor: bool,
    pub friction: f32,
}

pub fn generate_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TiledMap(asset_server.load("tilemap.tmx")));
}

pub fn on_add_environment_layer(
    trigger: On<Add, EnvironmentLayer>,
    mut commands: Commands,
    query: Query<&EnvironmentLayer>,
) {
    let layer_data = query.get(trigger.entity).unwrap();
    let mut entity_cmds = commands.entity(trigger.entity);

    // Give this specific layer its own physics body
    entity_cmds.insert((
        RigidBody::Static,
        Friction::new(layer_data.friction),
        CollisionLayers::new([GameLayer::Default], [GameLayer::Player, GameLayer::Enemy]),
    ));

    // If we checked the "is_sensor" box in Tiled, make it passable!
    if layer_data.is_sensor {
        entity_cmds.insert(Sensor);
    }
}
