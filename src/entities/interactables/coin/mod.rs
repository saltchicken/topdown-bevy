use super::{components::Interactable, events::InteractionEvent};
use crate::events::SpawnRequest;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Coin {
    pub value: u32,
}

#[derive(Clone)]
pub struct CoinPayload {
    pub value: u32,
}

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnRequest<CoinPayload>>()
           .add_systems(Update, (spawn_coins_from_requests, handle_coin_interactions));
    }
}

fn spawn_coins_from_requests(
    mut commands: Commands,
    mut requests: MessageReader<SpawnRequest<CoinPayload>>, 
) {
    for request in requests.read() {
        commands.spawn((
            Coin { value: request.payload.value },
            Interactable,
            Sprite {
                color: Color::srgb(1.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            Transform::from_xyz(request.position.x, request.position.y, 0.0),
            Collider::circle(10.0),
            Sensor,
        ));
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
