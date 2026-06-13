use super::{ActiveInteract, InteractedEvent};
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

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
#[reflect(Component, Default)]
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
    pub active_interact: ActiveInteract,
    pub sprite: Sprite,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl ChestBundle {
    pub fn new(config: &ChestConfig) -> Self {
        Self {
            active_interact: ActiveInteract,
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
            .register_type::<Chest>()
            .add_observer(on_add_chest)
            .add_observer(on_interact_chest);
    }
}

fn on_add_chest(trigger: On<Add, Chest>, mut commands: Commands, config: Res<ChestConfig>) {
    commands
        .entity(trigger.entity)
        .insert(ChestBundle::new(&config));
}

fn on_interact_chest(
    trigger: On<InteractedEvent>,
    mut chest_query: Query<(&mut Chest, &mut Sprite)>,
    config: Res<ChestConfig>,
) {
    if let Ok((mut chest, mut sprite)) = chest_query.get_mut(trigger.entity) {
        if !chest.is_open {
            chest.is_open = true;
            sprite.color = config.open_color;
            info!("Opened a chest and found {} gold!", chest.gold_content);
        }
    }
}
