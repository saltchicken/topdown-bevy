pub mod chest;
pub mod coin;
pub mod door;
pub mod light;

use avian2d::prelude::{Collider, CollisionEnd, CollisionStart, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct AutoCollect;

#[derive(Component, Reflect, Default)]
pub struct ActiveInteract;

#[derive(Component, Reflect, Default)]
pub struct ProximityTrigger;

#[derive(EntityEvent)]
pub struct CollectedEvent {
    pub entity: Entity,
    pub interactor: Entity,
}

#[derive(EntityEvent)]
pub struct InteractedEvent {
    pub entity: Entity,
    pub interactor: Entity,
}

#[derive(EntityEvent)]
pub struct ProximityEvent {
    pub entity: Entity,
    pub interactor: Entity,
    pub is_entering: bool,
}

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, active_interact_system)
            .add_observer(auto_collect_observer)
            .add_observer(proximity_start_observer)
            .add_observer(proximity_end_observer)
            .add_plugins(chest::ChestPlugin)
            .add_plugins(coin::CoinPlugin)
            .add_plugins(door::DoorPlugin)
            .add_plugins(light::LightPlugin);
    }
}

pub fn auto_collect_observer(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    player_query: Query<Entity, With<crate::entities::player::Player>>,
    interactable_query: Query<Entity, With<AutoCollect>>,
) {
    let collision = trigger.event();
    let e1 = collision.collider1;
    let e2 = collision.collider2;

    let (interactor, interactable) = if player_query.contains(e1) && interactable_query.contains(e2)
    {
        (e1, e2)
    } else if player_query.contains(e2) && interactable_query.contains(e1) {
        (e2, e1)
    } else {
        return;
    };

    commands
        .entity(interactable)
        .trigger(|entity| CollectedEvent { entity, interactor });
}

pub fn active_interact_system(
    mut commands: Commands,
    player_query: Query<
        (
            Entity,
            &Transform,
            &leafwing_input_manager::prelude::ActionState<crate::input::PlayerAction>,
        ),
        With<crate::entities::player::Player>,
    >,
    spatial_query: SpatialQuery,
    interactable_query: Query<Entity, With<ActiveInteract>>,
) {
    for (player_entity, transform, action_state) in &player_query {
        if action_state.just_pressed(&crate::input::PlayerAction::Interact) {
            let interact_radius = 48.0;
            let intersections = spatial_query.shape_intersections(
                &Collider::circle(interact_radius),
                transform.translation.truncate(),
                0.0,
                &SpatialQueryFilter::default(),
            );

            for entity in intersections {
                if interactable_query.contains(entity) {
                    commands.entity(entity).trigger(|entity| InteractedEvent {
                        entity,
                        interactor: player_entity,
                    });
                }
            }
        }
    }
}

pub fn proximity_start_observer(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    player_query: Query<Entity, With<crate::entities::player::Player>>,
    interactable_query: Query<Entity, With<ProximityTrigger>>,
) {
    let collision = trigger.event();
    let e1 = collision.collider1;
    let e2 = collision.collider2;
    let (interactor, interactable) = if player_query.contains(e1) && interactable_query.contains(e2)
    {
        (e1, e2)
    } else if player_query.contains(e2) && interactable_query.contains(e1) {
        (e2, e1)
    } else {
        return;
    };
    commands
        .entity(interactable)
        .trigger(|entity| ProximityEvent {
            entity,
            interactor,
            is_entering: true,
        });
}

pub fn proximity_end_observer(
    trigger: On<CollisionEnd>,
    mut commands: Commands,
    player_query: Query<Entity, With<crate::entities::player::Player>>,
    interactable_query: Query<Entity, With<ProximityTrigger>>,
) {
    let collision = trigger.event();
    let e1 = collision.collider1;
    let e2 = collision.collider2;
    let (interactor, interactable) = if player_query.contains(e1) && interactable_query.contains(e2)
    {
        (e1, e2)
    } else if player_query.contains(e2) && interactable_query.contains(e1) {
        (e2, e1)
    } else {
        return;
    };
    commands
        .entity(interactable)
        .trigger(|entity| ProximityEvent {
            entity,
            interactor,
            is_entering: false,
        });
}
