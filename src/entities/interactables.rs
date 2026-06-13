pub mod chest;
pub mod coin;

use avian2d::prelude::CollisionStart;
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Interactable;

#[derive(Component, Reflect, Default)]
pub struct Interactor;

#[derive(EntityEvent)]
pub struct InteractionEvent {
    pub entity: Entity,
    pub interactor: Entity,
}

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_interactions)
            .add_plugins(chest::ChestPlugin)
            .add_plugins(coin::CoinPlugin);
    }
}

pub fn detect_interactions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    interactor_query: Query<Entity, With<Interactor>>,
    interactable_query: Query<Entity, With<Interactable>>,
) {
    for collision in collision_events.read() {
        let e1 = collision.collider1;
        let e2 = collision.collider2;

        let (interactor, interactable) =
            if interactor_query.contains(e1) && interactable_query.contains(e2) {
                (e1, e2)
            } else if interactor_query.contains(e2) && interactable_query.contains(e1) {
                (e2, e1)
            } else {
                continue;
            };

        commands
            .entity(interactable)
            .trigger(|entity| InteractionEvent { entity, interactor });
    }
}
