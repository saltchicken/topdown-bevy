use crate::core::state::{GameState, GameplaySet};
use crate::core::utils::despawn_screen;
use crate::render::y_sort::YSort;
use crate::render::z_layers::ZLayer;
use crate::ui::loading::GameAssets;
use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize)]
pub struct EnemyConfig {
    pub speed: f32,
    pub scale: f32,
    pub sprite_size: u32,
    pub sprite_cols: u32,
    pub sprite_rows: u32,
    pub frame_duration: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        let config_str = include_str!("../../assets/data/enemy.ron");
        ron::from_str(config_str).expect("Failed to parse enemy.ron configuration")
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyConfig>()
            .add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<Enemy>)
            .add_systems(Update, animate_enemy.in_set(GameplaySet));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct EnemyAnimationTimer(Timer);

fn spawn_enemy(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<EnemyConfig>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.sprite_size, config.sprite_size),
        config.sprite_cols,
        config.sprite_rows,
        None,
        None,
    );
    let enemy_layout = texture_atlas_layouts.add(layout);

    // Spawn the enemy offset from the player
    commands.spawn((
        Sprite {
            image: game_assets.enemy_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: enemy_layout,
                index: 0, // Facing down
            }),
            ..default()
        },
        Transform::from_xyz(config.spawn_x, config.spawn_y, ZLayer::Entities.to_f32())
            .with_scale(Vec3::splat(config.scale)),
        Enemy,
        RigidBody::Dynamic,
        Collider::circle(8.0),
        Friction::new(0.0),
        Restitution::new(0.0),
        LinearDamping(10.0),
        LockedAxes::new().lock_rotation(),
        YSort(ZLayer::Entities),
        EnemyAnimationTimer(Timer::from_seconds(config.frame_duration, TimerMode::Repeating)),
    ));
}

fn animate_enemy(
    time: Res<Time>,
    mut query: Query<(&mut EnemyAnimationTimer, &mut Sprite), With<Enemy>>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                // Cycle through the first 4 frames (Row 0: Idle Down)
                if atlas.index >= 3 {
                    atlas.index = 0;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
