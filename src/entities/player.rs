use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::entities::interactables::Interactor;
use crate::input::PlayerAction;
use crate::physics::GameLayer;

#[derive(Component, Default, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Speed(pub f32);

#[derive(Resource)]
pub struct PlayerConfig {
    pub size: f32,
    pub base_speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            size: 40.0,
            base_speed: 300.0,
        }
    }
}

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Walking;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Running;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_observer(on_idle_enter)
            .add_observer(on_walking_enter)
            .add_systems(FixedUpdate, on_walking_update)
            .add_observer(on_running_enter)
            .add_systems(FixedUpdate, on_running_update)
            .add_systems(Startup, setup_player);
    }
}

fn on_idle_enter(trigger: On<Add, Idle>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 1.0, 0.0);
    }
}

fn on_walking_enter(trigger: On<Add, Walking>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(1.0, 0.0, 0.0);
    }
}

fn on_walking_update(
    mut query: Query<(&mut LinearVelocity, &ActionState<PlayerAction>, &Speed), With<Walking>>,
) {
    for (mut velocity, action_state, speed) in &mut query {
        let direction = action_state.axis_pair(&PlayerAction::Move);
        if direction.length_squared() > 0.0 {
            velocity.0 = direction.normalize() * speed.0;
        }
    }
}

fn on_running_enter(trigger: On<Add, Running>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 0.0, 1.0);
    }
}

fn on_running_update(
    mut query: Query<(&mut LinearVelocity, &ActionState<PlayerAction>, &Speed), With<Running>>,
) {
    for (mut velocity, action_state, speed) in &mut query {
        let direction = action_state.axis_pair(&PlayerAction::Move);
        if direction.length_squared() > 0.0 {
            velocity.0 = direction.normalize() * (speed.0 * 2.0);
        }
    }
}

pub fn is_walking(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
        && !action_state.pressed(&PlayerAction::Run)
}

pub fn is_running(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
        && action_state.pressed(&PlayerAction::Run)
}

pub fn is_idle(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() == 0.0
}

pub fn setup_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn((
        (
            Player,
            Interactor,
            Speed(config.base_speed),
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(config.size)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
        (
            Idle,
            StateMachine::default()
                .trans::<Idle, _>(is_walking, Walking)
                .trans::<Idle, _>(is_running, Running)
                .trans::<Walking, _>(is_idle, Idle)
                .trans::<Walking, _>(is_running, Running)
                .trans::<Running, _>(is_idle, Idle)
                .trans::<Running, _>(is_walking, Walking),
            PlayerAction::default_input_map(),
            ActionState::<PlayerAction>::default(),
        ),
        (
            RigidBody::Dynamic,
            Collider::rectangle(config.size, config.size),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::default(),
            LinearDamping(5.0),
            CollisionEventsEnabled,
            CollisionLayers::new([GameLayer::Player], [GameLayer::Default, GameLayer::Interactable]),
        ),
    ));
}
