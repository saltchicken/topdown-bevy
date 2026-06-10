use super::{components::Interactable, events::InteractionEvent};
use avian2d::prelude::*;
use bevy::prelude::*;

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
}

impl CoinBundle {
    pub fn new(value: u32, position: Vec2) -> Self {
        Self {
            coin: Coin { value },
            interactable: Interactable,
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            collider: Collider::circle(10.0),
            sensor: Sensor,
        }
    }
}

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_coin_interactions);
    }
}

fn handle_coin_interactions(
    mut commands: Commands,
    mut events: MessageReader<InteractionEvent>,
    coin_query: Query<&Coin>,
) {
    for event in events.read() {
        if let Ok(coin) = coin_query.get(event.interactable) {
            info!("Collected a coin worth {}!", coin.value);
            commands.entity(event.interactable).despawn();
        }
    }
}
