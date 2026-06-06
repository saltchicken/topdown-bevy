use crate::core::camera::{CameraFollow, MainCamera};
use crate::core::input::GameAction;
use crate::core::state::{GameState, GameplaySet};
use crate::core::utils::despawn_screen;
use crate::render::y_sort::YSort;
use crate::render::z_layers::ZLayer;
use crate::ui::loading::GameAssets;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize)]
pub struct PlayerConfig {
    pub speed: f32,
    pub scale: f32,
    pub sprite_size: u32,
    pub sprite_cols: u32,
    pub sprite_rows: u32,
    pub idle_frame_duration: f32,
    pub walk_frame_duration: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        let config_str = include_str!("../../assets/data/player.ron");
        ron::from_str(config_str).expect("Failed to parse player.ron configuration")
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<Player>)
            .add_systems(
                Update,
                (player_movement, player_animation_controller, animate_sprite).in_set(GameplaySet),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, PartialEq, Clone, Copy)]
enum PlayerAnimationState {
    IdleDown,
    IdleLeft,
    IdleUp,
    IdleRight,
    WalkDown,
    WalkLeft,
    WalkUp,
    WalkRight,
}

impl PlayerAnimationState {
    // Both 4x4 sprite sheets follow the exact same layout mappings
    fn indices(&self) -> (usize, usize) {
        match self {
            Self::IdleDown | Self::WalkDown => (0, 3),     // Row 0
            Self::IdleLeft | Self::WalkLeft => (4, 7),     // Row 1
            Self::IdleUp | Self::WalkUp => (8, 11),        // Row 2
            Self::IdleRight | Self::WalkRight => (12, 15), // Row 3
        }
    }

    // Helper function to figure out if we are currently walking
    fn is_walk(&self) -> bool {
        matches!(
            self,
            Self::WalkDown | Self::WalkLeft | Self::WalkUp | Self::WalkRight
        )
    }
}

fn setup_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<PlayerConfig>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.sprite_size, config.sprite_size),
        config.sprite_cols,
        config.sprite_rows,
        None,
        None,
    );
    let player_layout = texture_atlas_layouts.add(layout);

    // Spawn the player with the idle texture by default
    let player_entity = commands.spawn((
        Sprite {
            image: game_assets.player_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, ZLayer::Entities.to_f32()).with_scale(Vec3::splat(config.scale)),
        Player,
        YSort(ZLayer::Entities),
        PlayerAnimationState::IdleDown,
        AnimationTimer(Timer::from_seconds(
            config.idle_frame_duration,
            TimerMode::Repeating,
        )),
    )).id();

    if let Ok(camera_entity) = camera_query.single() {
        commands.entity(camera_entity).insert(CameraFollow {
            target: player_entity,
            decay_rate: 2.0,
        });
    }
}

// Logic Only: Reads inputs, updates Transform, and sets the intended PlayerAnimationState
fn player_movement(
    mut query: Query<(
        &Player,
        &mut Transform,
        &mut PlayerAnimationState,
    )>,
    action_state: Res<ActionState<GameAction>>,
    time: Res<Time>,
    config: Res<PlayerConfig>,
) {
    let Ok((_player, mut player_transform, mut animation_state)) = query.single_mut()
    else {
        return;
    };

    // Default to idle based on current facing direction
    let mut new_state = match *animation_state {
        PlayerAnimationState::WalkDown | PlayerAnimationState::IdleDown => {
            PlayerAnimationState::IdleDown
        }
        PlayerAnimationState::WalkUp | PlayerAnimationState::IdleUp => PlayerAnimationState::IdleUp,
        PlayerAnimationState::WalkLeft | PlayerAnimationState::IdleLeft => {
            PlayerAnimationState::IdleLeft
        }
        PlayerAnimationState::WalkRight | PlayerAnimationState::IdleRight => {
            PlayerAnimationState::IdleRight
        }
    };

    let axis = action_state.clamped_axis_pair(&GameAction::Move);
    let mut direction = axis.extend(0.0);

    if direction != Vec3::ZERO {
        if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                new_state = PlayerAnimationState::WalkRight;
            } else {
                new_state = PlayerAnimationState::WalkLeft;
            }
        } else {
            if direction.y > 0.0 {
                new_state = PlayerAnimationState::WalkUp;
            } else {
                new_state = PlayerAnimationState::WalkDown;
            }
        }
    }

    // Mutate the state only if it actually changed to prevent triggering Change Detection every frame
    if *animation_state != new_state {
        *animation_state = new_state;
    }

    // Apply movement
    direction = direction.normalize_or_zero();

    player_transform.translation += direction * config.speed * time.delta_secs();
}

// Visuals Only: Listens for changes to the animation state and updates visual components
fn player_animation_controller(
    mut query: Query<
        (&PlayerAnimationState, &mut Sprite, &mut AnimationTimer),
        (With<Player>, Changed<PlayerAnimationState>),
    >,
    animations: Res<GameAssets>,
    config: Res<PlayerConfig>,
) {
    for (state, mut sprite, mut timer) in &mut query {
        // Swap out the underlying sprite sheet image if we are crossing action boundaries
        if state.is_walk() {
            sprite.image = animations.player_walk.clone();
        } else {
            sprite.image = animations.player_idle.clone();
        }

        // Snap to the correct starting frame for the new state
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = state.indices().0;
        }

        // Adjust animation speed based on the action
        let duration = match state {
            PlayerAnimationState::IdleDown
            | PlayerAnimationState::IdleLeft
            | PlayerAnimationState::IdleUp
            | PlayerAnimationState::IdleRight => config.idle_frame_duration,
            _ => config.walk_frame_duration,
        };
        timer.set_duration(std::time::Duration::from_secs_f32(duration));
    }
}

// Progresses the frames for whatever animation is currently playing
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&PlayerAnimationState, &mut AnimationTimer, &mut Sprite)>,
) {
    for (state, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                let (start, end) = state.indices();

                if atlas.index < start || atlas.index >= end {
                    atlas.index = start;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
