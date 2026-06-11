use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct InteractionEvent {
    pub entity: Entity,
    pub interactor: Entity,
}
