use crate::core::state::{GameState, GameplaySet};
use crate::core::utils::despawn_screen;
use crate::render::y_sort::YSort;
use crate::render::z_layers::ZLayer;
use crate::ui::loading::GameAssets;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_enemy)
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
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4,
        4,
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
        Transform::from_xyz(100.0, 50.0, ZLayer::Entities.to_f32())
            .with_scale(Vec3::splat(1.5)), // Adjust scale as needed
        Enemy,
        YSort(ZLayer::Entities),
        EnemyAnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
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
