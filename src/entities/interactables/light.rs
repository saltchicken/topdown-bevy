use super::{ProximityEvent, ProximityTrigger};
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct LightConfig {
    pub size: f32,
    pub off_color: Color,
    pub on_color: Color,
}

impl Default for LightConfig {
    fn default() -> Self {
        Self {
            size: 20.0,
            off_color: Color::srgb(0.2, 0.2, 0.2),
            on_color: Color::srgb(1.0, 1.0, 0.8),
        }
    }
}

#[derive(Component, Reflect)]
pub struct Light {
    pub is_on: bool,
}

impl Default for Light {
    fn default() -> Self {
        Self { is_on: false }
    }
}

#[derive(Bundle)]
pub struct LightBundle {
    pub light: Light,
    pub proximity_trigger: ProximityTrigger,
    pub sprite: Sprite,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collision_layers: CollisionLayers,
}

impl LightBundle {
    pub fn new(config: &LightConfig) -> Self {
        Self {
            light: Light { is_on: false },
            proximity_trigger: ProximityTrigger,
            sprite: Sprite {
                color: config.off_color,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            rigid_body: RigidBody::Static,
            collider: Collider::circle(config.size * 5.0), // Large sensor collider
            sensor: Sensor,
            collision_layers: CollisionLayers::new([GameLayer::Interactable], [GameLayer::Player]),
        }
    }
}

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LightConfig>()
            .add_observer(handle_light_proximity);
    }
}

fn handle_light_proximity(
    trigger: On<ProximityEvent>,
    mut light_query: Query<(&mut Light, &mut Sprite)>,
    config: Res<LightConfig>,
) {
    let event = trigger.event();
    if let Ok((mut light, mut sprite)) = light_query.get_mut(trigger.entity) {
        light.is_on = event.is_entering;
        sprite.color = if light.is_on {
            config.on_color
        } else {
            config.off_color
        };
        info!(
            "Light proximity: {}",
            if light.is_on { "ENTERED" } else { "EXITED" }
        );
    }
}
