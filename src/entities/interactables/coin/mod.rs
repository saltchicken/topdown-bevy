use avian2d::prelude::*;
use bevy::prelude::*;
use super::{Interactable, InteractionEvent};

#[derive(Component, Default, Reflect)]
pub struct Coin {
    pub value: u32,
}

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_coins)
           .add_systems(Update, handle_coin_interactions);
    }
}

fn setup_coins(mut commands: Commands) {
    // Spawning a test coin
    commands.spawn((
        Coin { value: 1 },
        Interactable, // Add the generic Interactable tag
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        Transform::from_xyz(150.0, 100.0, 0.0),
        Collider::circle(10.0),
        Sensor,
    ));
}

fn handle_coin_interactions(
    mut commands: Commands,
    mut events: MessageReader<InteractionEvent>,
    coin_query: Query<&Coin>,
) {
    for event in events.read() {
        // If the entity that was interacted with is a Coin
        if let Ok(coin) = coin_query.get(event.0) {
            info!("Collected a coin worth {}!", coin.value);
            
            // Despawn the coin
            commands.entity(event.0).despawn();
            
            // Future: You could emit a ScoreUpdateEvent(coin.value) here
        }
    }
}
