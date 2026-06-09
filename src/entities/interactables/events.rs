use bevy::prelude::*;

#[derive(Message)]
pub struct InteractionEvent {
    pub interactor: Entity,
    pub interactable: Entity,
}
