use super::{ActiveInteract, InteractedEvent};
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct DoorConfig {
    pub size: f32,
    pub closed_color: Color,
    pub open_color: Color,
}

impl Default for DoorConfig {
    fn default() -> Self {
        Self {
            size: 32.0,
            closed_color: Color::srgb(0.6, 0.3, 0.1),
            open_color: Color::srgba(0.6, 0.3, 0.1, 0.3),
        }
    }
}

#[derive(Component, Reflect)]
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
    pub door: Door,
    pub active_interact: ActiveInteract,
    pub sprite: Sprite,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl DoorBundle {
    pub fn new(config: &DoorConfig) -> Self {
        Self {
            door: Door { is_open: false },
            active_interact: ActiveInteract,
            sprite: Sprite {
                color: config.closed_color,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(config.size, config.size),
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
        app.init_resource::<DoorConfig>()
            .add_observer(on_interact_door);
    }
}

fn on_interact_door(
    trigger: On<InteractedEvent>,
    mut commands: Commands,
    mut door_query: Query<(&mut Door, &mut Sprite)>,
    config: Res<DoorConfig>,
) {
    if let Ok((mut door, mut sprite)) = door_query.get_mut(trigger.entity) {
        door.is_open = !door.is_open;
        if door.is_open {
            sprite.color = config.open_color;
            commands.entity(trigger.entity).insert(Sensor);
            info!("Opened the door.");
        } else {
            sprite.color = config.closed_color;
            commands.entity(trigger.entity).remove::<Sensor>();
            info!("Closed the door.");
        }
    }
}
