use avian2d::prelude::CollisionStart;
use bevy::prelude::*;

use super::{
    components::{Interactable, Interactor},
    events::InteractionEvent,
};

pub fn detect_interactions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    interactor_query: Query<Entity, With<Interactor>>,
    interactable_query: Query<Entity, With<Interactable>>,
) {
    for collision in collision_events.read() {
        let e1 = collision.collider1;
        let e2 = collision.collider2;

        // Determine the correct assignment, or skip if neither condition matches
        let (interactor, interactable) = 
            if interactor_query.contains(e1) && interactable_query.contains(e2) {
                (e1, e2)
            } else if interactor_query.contains(e2) && interactable_query.contains(e1) {
                (e2, e1)
            } else {
                continue; // Move on to the next collision event
            };

        commands.entity(interactable).trigger(|entity| InteractionEvent { entity, interactor });
    }
}
