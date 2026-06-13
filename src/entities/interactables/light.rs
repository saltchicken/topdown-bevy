use super::Interactable;
use crate::entities::player::Player;
use crate::input::PlayerAction;
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
    pub interactable: Interactable,
    pub sprite: Sprite,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl LightBundle {
    pub fn new(config: &LightConfig) -> Self {
        Self {
            light: Light { is_on: false },
            interactable: Interactable,
            sprite: Sprite {
                color: config.off_color,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(config.size, config.size),
            collision_layers: CollisionLayers::new([GameLayer::Interactable], [GameLayer::Player]),
        }
    }
}

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LightConfig>()
            .add_systems(Update, handle_light_interactions);
    }
}

fn handle_light_interactions(
    mut light_query: Query<(&mut Light, &mut Sprite, &Transform)>,
    player_query: Query<(&ActionState<PlayerAction>, &Transform), With<Player>>,
    config: Res<LightConfig>,
) {
    let interact_range = config.size * 2.5;

    for (action_state, player_transform) in &player_query {
        if action_state.just_pressed(&PlayerAction::Interact) {
            for (mut light, mut sprite, light_transform) in &mut light_query {
                if player_transform.translation.distance(light_transform.translation) <= interact_range {
                    light.is_on = !light.is_on;
                    sprite.color = if light.is_on {
                        config.on_color
                    } else {
                        config.off_color
                    };
                    info!("Turned light {}!", if light.is_on { "ON" } else { "OFF" });
                }
            }
        }
    }
}
