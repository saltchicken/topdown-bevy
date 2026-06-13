use super::{ActiveInteract, InteractedEvent};
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Door {
    pub is_open: bool,
}

impl Default for Door {
    fn default() -> Self {
        Self { is_open: false }
    }
}

#[derive(Bundle)]
pub struct DoorBundle {
    pub active_interact: ActiveInteract,
    pub rigid_body: RigidBody,
    pub collision_layers: CollisionLayers,
}

impl DoorBundle {
    pub fn new() -> Self {
        Self {
            active_interact: ActiveInteract,
            rigid_body: RigidBody::Static,
            collision_layers: CollisionLayers::new(
                [GameLayer::Default, GameLayer::Interactable],
                [GameLayer::Player, GameLayer::Enemy],
            ),
        }
    }
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Door>()
            .add_observer(on_add_door)
            .add_observer(on_interact_door);
    }
}

fn on_add_door(trigger: On<Add, Door>, mut commands: Commands) {
    commands.entity(trigger.entity).insert(DoorBundle::new());
}

fn on_interact_door(
    trigger: On<InteractedEvent>,
    mut commands: Commands,
    mut door_query: Query<(&mut Door, &mut Visibility)>, // Query Visibility instead of Sprite
) {
    if let Ok((mut door, mut visibility)) = door_query.get_mut(trigger.entity) {
        door.is_open = !door.is_open;
        if door.is_open {
            *visibility = Visibility::Hidden; // Hide the door graphic
            commands.entity(trigger.entity).insert(Sensor); // Make it passable
            info!("Opened the door.");
        } else {
            *visibility = Visibility::Inherited; // Show the door graphic
            commands.entity(trigger.entity).remove::<Sensor>(); // Make it solid again
            info!("Closed the door.");
        }
    }
}
