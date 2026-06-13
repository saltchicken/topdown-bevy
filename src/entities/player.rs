pub mod states;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::input::PlayerAction;
use crate::physics::GameLayer;

use self::states::PlayerStatePlugin;
use self::states::dashing::*;
use self::states::idle::*;
use self::states::running::*;
use self::states::walking::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Speed(pub f32);

#[derive(Resource)]
pub struct PlayerConfig {
    pub size: f32,
    pub base_speed: f32,
    pub color_idle: Color,
    pub color_running: Color,
    pub color_walking: Color,
    pub color_dashing: Color,
    pub walk_speed_multiplier: f32,
    pub dash_speed_multiplier: f32,
    pub dash_duration: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            size: 24.0,
            base_speed: 300.0,
            color_idle: Color::srgb(0.0, 1.0, 0.0),
            color_running: Color::srgb(1.0, 0.0, 0.0),
            color_walking: Color::srgb(0.0, 1.0, 1.0),
            color_dashing: Color::srgb(0.0, 0.0, 1.0),
            walk_speed_multiplier: 0.5,
            dash_speed_multiplier: 5.0,
            dash_duration: 0.2,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub speed: Speed,
    pub sprite: Sprite,
    pub idle: Idle,
    pub state_machine: StateMachine,
    pub input_map: InputMap<PlayerAction>,
    pub action_state: ActionState<PlayerAction>,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub linear_velocity: LinearVelocity,
    pub linear_damping: LinearDamping,
    pub collision_events: CollisionEventsEnabled,
    pub collision_layers: CollisionLayers,
}

impl PlayerBundle {
    pub fn new(config: &PlayerConfig) -> Self {
        Self {
            speed: Speed(config.base_speed),
            sprite: Sprite {
                color: config.color_idle,
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            idle: Idle,
            state_machine: StateMachine::default()
                .trans::<Idle, _>(is_running, Running)
                .trans::<Idle, _>(is_walking, Walking)
                .trans::<Idle, _>(is_dashing, Dashing)
                .trans::<Running, _>(is_idle, Idle)
                .trans::<Running, _>(is_walking, Walking)
                .trans::<Running, _>(is_dashing, Dashing)
                .trans::<Walking, _>(is_idle, Idle)
                .trans::<Walking, _>(is_running, Running)
                .trans::<Walking, _>(is_dashing, Dashing)
                .trans::<Dashing, _>(IntoTrigger::and(dash_timer_finished, is_idle), Idle)
                .trans::<Dashing, _>(IntoTrigger::and(dash_timer_finished, is_running), Running)
                .trans::<Dashing, _>(IntoTrigger::and(dash_timer_finished, is_walking), Walking),
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::<PlayerAction>::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(config.size / 2.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            linear_velocity: LinearVelocity::default(),
            linear_damping: LinearDamping(5.0),
            collision_events: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                [GameLayer::Player],
                [GameLayer::Default, GameLayer::Interactable],
            ),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .register_type::<Player>()
            .add_observer(on_add_player)
            .add_plugins(PlayerStatePlugin)
            .add_systems(Update, camera_follow_player);
    }
}

fn on_add_player(trigger: On<Add, Player>, mut commands: Commands, config: Res<PlayerConfig>) {
    commands
        .entity(trigger.entity)
        .insert(PlayerBundle::new(&config));
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
