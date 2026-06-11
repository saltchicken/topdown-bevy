use super::{Interactable, InteractionEvent};
use avian2d::prelude::*;
use bevy::prelude::*;
use crate::physics::GameLayer;

#[derive(Resource)]
pub struct CoinConfig {
    pub default_value: u32,
    pub size: f32,
    pub collider_radius: f32,
    pub color: Color,
}

impl Default for CoinConfig {
    fn default() -> Self {
        Self {
            default_value: 1,
            size: 20.0,
            collider_radius: 10.0,
            color: Color::srgb(1.0, 1.0, 0.0),
        }
    }
}

#[derive(Component, Default, Reflect)]
pub struct Coin {
    pub value: u32,
}

#[derive(Bundle)]
pub struct CoinBundle {
    pub coin: Coin,
    pub interactable: Interactable,
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collision_layers: CollisionLayers,
}

impl CoinBundle {
    pub fn new(value: u32, position: Vec2, config: &CoinConfig) -> Self {
        Self {
            coin: Coin { value },
            interactable: Interactable,
            sprite: Sprite {
                color: config.color,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            collider: Collider::circle(config.collider_radius),
            sensor: Sensor,
            collision_layers: CollisionLayers::new([GameLayer::Interactable], [GameLayer::Player]),
        }
    }
}

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoinConfig>()
            .add_observer(handle_coin_interactions);
    }
}

fn handle_coin_interactions(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    coin_query: Query<&Coin>,
) {
    let interactable = trigger.entity;
    if let Ok(coin) = coin_query.get(interactable) {
        info!("Collected a coin worth {}!", coin.value);
        commands.entity(interactable).despawn();
    }
}
