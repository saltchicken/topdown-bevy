use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::entities::player::{PlayerConfig, Speed};
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
    config: Res<PlayerConfig>,
) {
    if let Ok((mut sprite, mut velocity, action_state, speed)) = query.get_mut(trigger.entity) {
        sprite.color = config.color_dashing;
        let direction = action_state.axis_pair(&PlayerAction::Move);
        if direction.length_squared() > 0.0 {
            // Apply a quick burst of speed initially
            velocity.0 = direction.normalize() * (speed.0 * config.dash_speed_multiplier);
        }
        commands
            .entity(trigger.entity)
            .insert(DashTimer(Timer::from_seconds(config.dash_duration, TimerMode::Once)));
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
    crate::input::MovementIntention::evaluate(action_state) == crate::input::MovementIntention::Dashing
}

pub fn dash_timer_finished(In(entity): In<Entity>, query: Query<&DashTimer>) -> bool {
    !query.contains(entity)
}
