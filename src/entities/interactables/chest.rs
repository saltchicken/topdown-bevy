use super::Interactable;
use crate::entities::player::Player;
use crate::input::PlayerAction;
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Resource)]
pub struct ChestConfig {
    pub size: f32,
    pub closed_color: Color,
    pub open_color: Color,
}

impl Default for ChestConfig {
    fn default() -> Self {
        Self {
            size: 24.0,
            closed_color: Color::srgb(0.4, 0.2, 0.0), // Brownish
            open_color: Color::srgb(1.0, 0.8, 0.0),   // Goldish
        }
    }
}

#[derive(Component, Reflect)]
pub struct Chest {
    pub is_open: bool,
    pub gold_content: u32,
}

impl Default for Chest {
    fn default() -> Self {
        Self {
            is_open: false,
            gold_content: 10,
        }
    }
}

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub interactable: Interactable,
    pub sprite: Sprite,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl ChestBundle {
    pub fn new(gold_content: u32, config: &ChestConfig) -> Self {
        Self {
            chest: Chest {
                is_open: false,
                gold_content,
            },
            interactable: Interactable,
            sprite: Sprite {
                color: config.closed_color,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(config.size, config.size),
            collision_layers: CollisionLayers::new([GameLayer::Interactable], [GameLayer::Player]),
        }
    }
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChestConfig>()
            .add_systems(Update, handle_chest_interactions);
    }
}

fn handle_chest_interactions(
    mut chest_query: Query<(&mut Chest, &mut Sprite, &Transform)>,
    player_query: Query<(&ActionState<PlayerAction>, &Transform), With<Player>>,
    config: Res<ChestConfig>,
) {
    let interact_range = config.size * 2.0;

    for (action_state, player_transform) in &player_query {
        if action_state.just_pressed(&PlayerAction::Interact) {
            for (mut chest, mut sprite, chest_transform) in &mut chest_query {
                if player_transform.translation.distance(chest_transform.translation) <= interact_range {
                    if !chest.is_open {
                        chest.is_open = true;
                        sprite.color = config.open_color;
                        info!("Opened a chest and found {} gold!", chest.gold_content);
                    }
                }
            }
        }
    }
}
