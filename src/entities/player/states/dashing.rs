use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::entities::player::Speed;
use crate::input::PlayerAction;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Dashing;

#[derive(Component)]
pub struct DashTimer(pub Timer);

pub fn on_enter(
    trigger: On<Add, Dashing>,
    mut commands: Commands,
    mut query: Query<(&mut Sprite, &mut LinearVelocity, &ActionState<PlayerAction>, &Speed)>,
) {
    if let Ok((mut sprite, mut velocity, action_state, speed)) = query.get_mut(trigger.entity) {
        sprite.color = Color::srgb(0.0, 0.0, 1.0);
        let direction = action_state.axis_pair(&PlayerAction::Move);
        if direction.length_squared() > 0.0 {
            // Apply a quick burst of speed initially
            velocity.0 = direction.normalize() * (speed.0 * 5.0);
        }
        commands
            .entity(trigger.entity)
            .insert(DashTimer(Timer::from_seconds(0.2, TimerMode::Once)));
    }
}

pub fn on_update(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DashTimer), With<Dashing>>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).remove::<DashTimer>();
        }
    }
}

pub fn is_dashing(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
        && action_state.just_pressed(&PlayerAction::Dash)
}

pub fn done_dashing_and_idle(
    In(entity): In<Entity>,
    dash_query: Query<&DashTimer>,
    action_query: Query<&ActionState<PlayerAction>>,
) -> bool {
    if dash_query.contains(entity) {
        return false;
    }
    let Ok(action_state) = action_query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() == 0.0
}

pub fn done_dashing_and_running(
    In(entity): In<Entity>,
    dash_query: Query<&DashTimer>,
    action_query: Query<&ActionState<PlayerAction>>,
) -> bool {
    if dash_query.contains(entity) {
        return false;
    }
    let Ok(action_state) = action_query.get(entity) else {
        return false;
    };
    action_state.axis_pair(&PlayerAction::Move).length_squared() > 0.0
}
