use crate::core::camera::{CameraFollow, MainCamera};
use crate::core::input::GameAction;
use crate::core::state::{GameState, GameplaySet};
use crate::core::utils::despawn_screen;
use crate::entities::enemy::Enemy;
use crate::render::y_sort::YSort;
use crate::render::z_layers::ZLayer;
use crate::ui::loading::GameAssets;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use leafwing_input_manager::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Clone)]
pub struct PlayerConfig {
    pub acceleration: f32,
    pub max_speed: f32,
    pub scale: f32,
    pub sprite_size: u32,
    pub sprite_cols: u32,
    pub sprite_rows: u32,
    pub idle_frame_duration: f32,
    pub walk_frame_duration: f32,
}

#[derive(Message)]
pub struct PlayerTouchedEnemyEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerTouchedEnemyEvent>()
            .add_plugins(
                bevy_common_assets::ron::RonAssetPlugin::<PlayerConfig>::new(&["player.ron"]),
            )
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<Player>)
            .add_systems(
                Update,
                (
                    read_player_input,
                    update_player_state,
                    player_animation_controller,
                    animate_sprite,
                    handle_player_enemy_collisions,
                    handle_hit_duration,
                )
                    .in_set(GameplaySet),
            )
            .add_systems(FixedUpdate, apply_player_movement.in_set(GameplaySet));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
struct MovementIntent(Vec2);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Hit {
    pub timer: Timer,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

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
    player_configs: Res<Assets<PlayerConfig>>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    let config = player_configs
        .get(&game_assets.player_config)
        .expect("Player config should be loaded");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(config.sprite_size, config.sprite_size),
        config.sprite_cols,
        config.sprite_rows,
        None,
        None,
    );
    let player_layout = texture_atlas_layouts.add(layout);

    // Spawn the player with the idle texture by default
    let player_entity = commands
        .spawn((
            Sprite {
                image: game_assets.player_idle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_layout,
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, ZLayer::Entities.to_f32())
                .with_scale(Vec3::splat(config.scale)),
            Player,
            MovementIntent::default(),
            RigidBody::Dynamic,
            Collider::circle(8.0),
            CollisionEventsEnabled,
            Friction::new(0.0),
            Restitution::new(0.0),
            LinearVelocity::default(),
            LinearDamping(10.0),
            LockedAxes::new().lock_rotation(),
            YSort(ZLayer::Entities),
            PlayerAnimationState::IdleDown,
            AnimationTimer(Timer::from_seconds(
                config.idle_frame_duration,
                TimerMode::Repeating,
            )),
        ))
        .id();

    if let Ok(camera_entity) = camera_query.single() {
        commands.entity(camera_entity).insert(CameraFollow {
            target: player_entity,
            decay_rate: 2.0,
        });
    }
}

// Read player intention from input
fn read_player_input(
    mut query: Query<&mut MovementIntent, With<Player>>,
    action_state: Res<ActionState<GameAction>>,
) {
    let Ok(mut intent) = query.single_mut() else {
        return;
    };
    let axis = action_state.clamped_axis_pair(&GameAction::Move);
    intent.0 = axis.clamp_length_max(1.0);
}

// Apply movement logic and physics in FixedUpdate
fn apply_player_movement(
    mut query: Query<(&MovementIntent, &mut LinearVelocity), With<Player>>,
    game_assets: Res<GameAssets>,
    player_configs: Res<Assets<PlayerConfig>>,
    time: Res<Time>,
) {
    let Ok((intent, mut velocity)) = query.single_mut() else {
        return;
    };
    let config = player_configs
        .get(&game_assets.player_config)
        .expect("Player config should be loaded");

    if intent.0.length_squared() > 0.0 {
        // Apply acceleration over time
        velocity.0 += intent.0 * config.acceleration * time.delta_secs();

        // Prevent the player from exceeding the speed limit
        velocity.0 = velocity.0.clamp_length_max(config.max_speed);
    }
}

// System 2: Observes velocity and updates the animation state
fn update_player_state(
    mut query: Query<(&LinearVelocity, &mut PlayerAnimationState), With<Player>>,
) {
    for (velocity, mut state) in &mut query {
        let is_moving = velocity.0.length_squared() > 0.01;

        let new_state = if is_moving {
            // Determine direction based on the strongest velocity axis
            if velocity.0.x.abs() > velocity.0.y.abs() {
                if velocity.0.x > 0.0 {
                    PlayerAnimationState::WalkRight
                } else {
                    PlayerAnimationState::WalkLeft
                }
            } else {
                if velocity.0.y > 0.0 {
                    PlayerAnimationState::WalkUp
                } else {
                    PlayerAnimationState::WalkDown
                }
            }
        } else {
            // Fallback to idle based on current state
            match *state {
                PlayerAnimationState::WalkDown => PlayerAnimationState::IdleDown,
                PlayerAnimationState::WalkUp => PlayerAnimationState::IdleUp,
                PlayerAnimationState::WalkLeft => PlayerAnimationState::IdleLeft,
                PlayerAnimationState::WalkRight => PlayerAnimationState::IdleRight,
                _ => *state, // Keep current idle state
            }
        };

        if *state != new_state {
            *state = new_state;
        }
    }
}

// Visuals Only: Listens for changes to the animation state and updates visual components
fn player_animation_controller(
    mut query: Query<
        (&PlayerAnimationState, &mut Sprite, &mut AnimationTimer),
        (With<Player>, Changed<PlayerAnimationState>),
    >,
    animations: Res<GameAssets>,
    player_configs: Res<Assets<PlayerConfig>>,
) {
    let config = player_configs
        .get(&animations.player_config)
        .expect("Player config should be loaded");
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

fn handle_player_enemy_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    player_query: Query<(Entity, Option<&Hit>), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut ev_player_touched_enemy: MessageWriter<PlayerTouchedEnemyEvent>,
) {
    // We use get_single() here to safely unwrap the tuple
    let Ok((player_entity, hit_component)) = player_query.single() else {
        return;
    };

    for collision in collision_events.read() {
        if (collision.collider1 == player_entity && enemy_query.contains(collision.collider2))
            || (collision.collider2 == player_entity && enemy_query.contains(collision.collider1))
        {
            // Only apply the hit logic if the player doesn't already have the component
            if hit_component.is_none() {
                info!("Player touched the enemy!");
                ev_player_touched_enemy.write(PlayerTouchedEnemyEvent);

                // Give the player the hit component
                commands.entity(player_entity).insert(Hit::default());
            }
        }
    }
}

fn handle_hit_duration(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hit)>,
) {
    for (entity, mut hit) in &mut query {
        hit.timer.tick(time.delta());

        if hit.timer.just_finished() {
            commands.entity(entity).remove::<Hit>();
            info!("Hit component removed!");
        }
    }
}
