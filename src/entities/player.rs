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
use seldom_state::prelude::*; // Import seldom_state
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
            .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<PlayerConfig>::new(&["player.ron"]))
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<Player>)
            .add_systems(
                Update,
                (
                    read_player_input,
                    update_facing,
                    player_animation_controller,
                    animate_sprite,
                    handle_player_enemy_collisions,
                    tick_stunned_timers, // Add our new timer system here
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

// --- seldom_state Components ---

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Walk;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Stunned {
    pub timer: Timer,
}

impl Default for Stunned {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

// Tick the stun timer in a normal system instead of inside the trigger
fn tick_stunned_timers(mut query: Query<&mut Stunned>, time: Res<Time>) {
    for mut stunned in &mut query {
        stunned.timer.tick(time.delta());
    }
}

// The trigger must be a read-only system
fn stunned_time_trigger(In(entity): In<Entity>, query: Query<&Stunned>) -> bool {
    query.get(entity).is_ok_and(|stunned| {
        stunned.timer.elapsed() >= stunned.timer.duration()
    })
}

// We separate Facing from the action state so we don't need 8 different state components
#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
pub enum Facing {
    #[default]
    Down,
    Left,
    Up,
    Right,
}

impl Facing {
    fn row_indices(&self) -> (usize, usize) {
        match self {
            Self::Down => (0, 3),
            Self::Left => (4, 7),
            Self::Up => (8, 11),
            Self::Right => (12, 15),
        }
    }
}

fn moving_trigger(In(entity): In<Entity>, query: Query<&MovementIntent>) -> bool {
    query.get(entity).is_ok_and(|intent| intent.0.length_squared() > 0.01)
}

fn stopped_trigger(In(entity): In<Entity>, query: Query<&MovementIntent>) -> bool {
    query.get(entity).is_ok_and(|intent| intent.0.length_squared() <= 0.01)
}

// --- Systems ---

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

    // Define the state machine
    let state_machine = StateMachine::default()
        .trans::<Idle, _>(moving_trigger, Walk)
        .trans::<Walk, _>(stopped_trigger, Idle)
        .trans::<Stunned, _>(stunned_time_trigger, Idle);

    // Split components into two bundles to avoid Bevy's 15-item tuple limit
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
            Facing::default(),
            RigidBody::Dynamic,
            Collider::circle(8.0),
            CollisionEventsEnabled,
            Friction::new(0.0),
            Restitution::new(0.0),
            LinearVelocity::default(),
            LinearDamping(10.0),
            LockedAxes::new().lock_rotation(),
        ))
        .insert((
            YSort(ZLayer::Entities),
            AnimationTimer(Timer::from_seconds(
                config.idle_frame_duration,
                TimerMode::Repeating,
            )),
            Idle,
            state_machine,
        ))
        .id();

    if let Ok(camera_entity) = camera_query.single() {
        commands.entity(camera_entity).insert(CameraFollow {
            target: player_entity,
            decay_rate: 2.0,
        });
    }
}

fn read_player_input(
    mut query: Query<&mut MovementIntent, (With<Player>, Without<Stunned>)>,
    action_state: Res<ActionState<GameAction>>,
) {
    let Ok(mut intent) = query.single_mut() else { return };
    let axis = action_state.clamped_axis_pair(&GameAction::Move);
    intent.0 = axis.clamp_length_max(1.0);
}

fn apply_player_movement(
    mut query: Query<(&MovementIntent, &mut LinearVelocity), (With<Player>, Without<Stunned>)>,
    game_assets: Res<GameAssets>,
    player_configs: Res<Assets<PlayerConfig>>,
    time: Res<Time>,
) {
    let Ok((intent, mut velocity)) = query.single_mut() else { return };
    let config = player_configs.get(&game_assets.player_config).unwrap();

    if intent.0.length_squared() > 0.0 {
        velocity.0 += intent.0 * config.acceleration * time.delta_secs();
        velocity.0 = velocity.0.clamp_length_max(config.max_speed);
    }
}

// Determines the Facing direction independently of the state
fn update_facing(mut query: Query<(&LinearVelocity, &mut Facing), With<Player>>) {
    for (velocity, mut facing) in &mut query {
        if velocity.0.length_squared() > 0.01 {
            // 1. Calculate what the facing direction SHOULD be
            let new_facing = if velocity.0.x.abs() > velocity.0.y.abs() {
                if velocity.0.x > 0.0 { Facing::Right } else { Facing::Left }
            } else {
                if velocity.0.y > 0.0 { Facing::Up } else { Facing::Down }
            };

            // 2. Only mutate the component if it's actually different
            if *facing != new_facing {
                *facing = new_facing;
            }
        }
    }
}

// Listens for structural changes (Added components) instead of checking enums
fn player_animation_controller(
    mut query: Query<
        (
            Has<Walk>, // Returns true if entity has the Walk component
            &Facing,
            &mut Sprite,
            &mut AnimationTimer,
        ),
        (
            With<Player>,
            Or<(Added<Idle>, Added<Walk>, Added<Stunned>, Changed<Facing>)>,
        ),
    >,
    animations: Res<GameAssets>,
    player_configs: Res<Assets<PlayerConfig>>,
) {
    let Ok((is_walking, facing, mut sprite, mut timer)) = query.single_mut() else { return };
    let config = player_configs.get(&animations.player_config).unwrap();

    if is_walking {
        sprite.image = animations.player_walk.clone();
        timer.set_duration(std::time::Duration::from_secs_f32(config.walk_frame_duration));
    } else {
        sprite.image = animations.player_idle.clone();
        timer.set_duration(std::time::Duration::from_secs_f32(config.idle_frame_duration));
    }

    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.index = facing.row_indices().0;
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Facing, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    for (facing, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                let (start, end) = facing.row_indices();
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
    player_query: Query<Entity, (With<Player>, Without<Stunned>)>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut ev_player_touched_enemy: MessageWriter<PlayerTouchedEnemyEvent>,
) {
    let Ok(player_entity) = player_query.single() else { return };

    for collision in collision_events.read() {
        if (collision.collider1 == player_entity && enemy_query.contains(collision.collider2))
            || (collision.collider2 == player_entity && enemy_query.contains(collision.collider1))
        {
            info!("Player touched the enemy!");
            ev_player_touched_enemy.write(PlayerTouchedEnemyEvent);

            // seldom_state allows us to "interrupt" the state machine by explicitly inserting
            // the state component and removing the others.
            commands.entity(player_entity)
                .insert(Stunned::default())
                .remove::<Idle>()
                .remove::<Walk>();
        }
    }
}
