use avian2d::prelude::CollisionStart;
use bevy::prelude::*;

use super::{
    components::{Interactable, Interactor},
    events::InteractionEvent,
};

pub fn detect_interactions(
    mut collision_events: MessageReader<CollisionStart>,
    interactor_query: Query<Entity, With<Interactor>>,
    interactable_query: Query<Entity, With<Interactable>>,
    mut ev_interaction: MessageWriter<InteractionEvent>,
) {
    for collision in collision_events.read() {
        let e1 = collision.collider1;
        let e2 = collision.collider2;

        // Check if Entity 1 interacted with Entity 2
        if interactor_query.contains(e1) && interactable_query.contains(e2) {
            ev_interaction.write(InteractionEvent {
                interactor: e1,
                interactable: e2,
            });
        }
        // Check if Entity 2 interacted with Entity 1
        else if interactor_query.contains(e2) && interactable_query.contains(e1) {
            ev_interaction.write(InteractionEvent {
                interactor: e2,
                interactable: e1,
            });
        }
    }
}
