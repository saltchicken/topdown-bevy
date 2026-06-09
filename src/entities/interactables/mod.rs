pub mod coin;

use avian2d::prelude::CollisionStart;
use bevy::prelude::*;
use crate::entities::player::components::Player;

#[derive(Component, Reflect)]
pub struct Interactable;

#[derive(Message)]
pub struct InteractionEvent(pub Entity);

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InteractionEvent>()
            .add_systems(Update, detect_interactions)
            .add_plugins(coin::CoinPlugin);
    }
}

fn detect_interactions(
    mut collision_events: MessageReader<CollisionStart>,
    player_query: Query<Entity, With<Player>>,
    interactable_query: Query<Entity, With<Interactable>>,
    mut ev_interaction: MessageWriter<InteractionEvent>,
) {
    // Safely get the player entity; return if it doesn't exist yet
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for collision in collision_events.read() {
        if collision.collider1 == player_entity && interactable_query.contains(collision.collider2) {
            ev_interaction.write(InteractionEvent(collision.collider2));
        } else if collision.collider2 == player_entity && interactable_query.contains(collision.collider1) {
            ev_interaction.write(InteractionEvent(collision.collider1));
        }
    }
}
