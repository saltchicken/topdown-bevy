use super::{AutoCollect, CollectedEvent};
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

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
            size: 10.0,
            collider_radius: 5.0,
            color: Color::srgb(1.0, 1.0, 0.0),
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Coin {
    pub value: u32,
}

#[derive(Bundle)]
pub struct CoinBundle {
    pub auto_collect: AutoCollect,
    pub sprite: Sprite,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collision_layers: CollisionLayers,
}

impl CoinBundle {
    pub fn new(config: &CoinConfig) -> Self {
        Self {
            auto_collect: AutoCollect,
            sprite: Sprite {
                color: config.color,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
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
            .register_type::<Coin>()
            .add_observer(on_add_coin)
            .add_observer(handle_coin_interactions);
    }
}

fn on_add_coin(trigger: On<Add, Coin>, mut commands: Commands, config: Res<CoinConfig>) {
    commands.entity(trigger.entity).insert(CoinBundle::new(&config));
}

fn handle_coin_interactions(
    trigger: On<CollectedEvent>,
    mut commands: Commands,
    coin_query: Query<&Coin>,
) {
    let interactable = trigger.entity;
    if let Ok(coin) = coin_query.get(interactable) {
        info!("Collected a coin worth {}!", coin.value);
        commands.entity(interactable).despawn();
    }
}
