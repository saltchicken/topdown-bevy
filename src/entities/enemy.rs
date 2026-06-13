pub mod states;

use avian2d::prelude::*;
use bevy::prelude::*;
use seldom_state::prelude::*;

use crate::entities::player::Player;
use crate::physics::GameLayer;

use self::states::EnemyStatePlugin;
use self::states::chase::*;
use self::states::idle::*;
use self::states::patrol::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Reflect)]
pub struct PatrolTimer(pub Timer);

#[derive(Component, Reflect)]
pub struct IdleTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyConfig {
    pub size: f32,
    pub patrol_speed: f32,
    pub chase_speed: f32,
    pub aggro_radius: f32,
    pub color_idle: Color,
    pub color_patrol: Color,
    pub color_chase: Color,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            size: 24.0,
            patrol_speed: 100.0,
            chase_speed: 200.0,
            aggro_radius: 250.0,
            color_idle: Color::srgb(0.5, 0.5, 0.5),
            color_patrol: Color::srgb(0.0, 0.0, 1.0),
            color_chase: Color::srgb(1.0, 0.0, 0.0),
        }
    }
}

pub fn player_in_aggro_range(
    In(entity): In<Entity>,
    enemy_query: Query<&Transform>,
    player_query: Query<&Transform, With<Player>>,
    config: Res<EnemyConfig>,
) -> bool {
    let Ok(enemy_transform) = enemy_query.get(entity) else {
        return false;
    };
    let Ok(player_transform) = player_query.single() else {
        return false;
    };
    enemy_transform
        .translation
        .distance(player_transform.translation)
        <= config.aggro_radius
}

pub fn player_out_of_aggro_range(
    In(entity): In<Entity>,
    enemy_query: Query<&Transform>,
    player_query: Query<&Transform, With<Player>>,
    config: Res<EnemyConfig>,
) -> bool {
    let Ok(enemy_transform) = enemy_query.get(entity) else {
        return false;
    };
    let Ok(player_transform) = player_query.single() else {
        return false;
    };
    enemy_transform
        .translation
        .distance(player_transform.translation)
        > config.aggro_radius
}

pub fn idle_timer_finished(In(entity): In<Entity>, query: Query<&IdleTimer>) -> bool {
    query
        .get(entity)
        .map(|t| t.0.is_finished())
        .unwrap_or(false)
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite: Sprite,
    pub idle: Idle,
    pub state_machine: StateMachine,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub linear_velocity: LinearVelocity,
    pub linear_damping: LinearDamping,
    pub collision_events: CollisionEventsEnabled,
    pub collision_layers: CollisionLayers,
    pub idle_timer: IdleTimer,
    pub patrol_timer: PatrolTimer,
    pub move_direction: MoveDirection,
}

impl EnemyBundle {
    pub fn new(config: &EnemyConfig) -> Self {
        Self {
            sprite: Sprite {
                color: config.color_idle,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            idle: Idle,
            state_machine: StateMachine::default()
                .trans::<Idle, _>(player_in_aggro_range, Chase)
                .trans::<Idle, _>(idle_timer_finished, Patrol)
                .trans::<Patrol, _>(player_in_aggro_range, Chase)
                .trans::<Chase, _>(player_out_of_aggro_range, Idle),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(config.size, config.size),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            linear_velocity: LinearVelocity::default(),
            linear_damping: LinearDamping(5.0),
            collision_events: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                [GameLayer::Enemy],
                [
                    GameLayer::Default,
                    GameLayer::Player,
                    GameLayer::Interactable,
                    GameLayer::Enemy,
                ],
            ),
            idle_timer: IdleTimer(Timer::from_seconds(2.0, TimerMode::Once)),
            patrol_timer: PatrolTimer(Timer::from_seconds(3.0, TimerMode::Repeating)),
            move_direction: MoveDirection(Vec2::new(1.0, 0.0)),
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyConfig>()
            .register_type::<Enemy>()
            .add_observer(on_add_enemy)
            .add_plugins(EnemyStatePlugin);
    }
}

fn on_add_enemy(trigger: On<Add, Enemy>, mut commands: Commands, config: Res<EnemyConfig>) {
    commands
        .entity(trigger.entity)
        .insert(EnemyBundle::new(&config));
}
