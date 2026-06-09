use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;
use self::states::{active::*, inactive::*};
use crate::input::PlayerAction;

pub mod states;

fn toggle_pressed(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    let Ok(action_state) = query.get(entity) else { return false; };
    action_state.just_pressed(&PlayerAction::Toggle)
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ActivePlugin, InactivePlugin))
            .add_systems(Startup, setup_player);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Inactive,
        StateMachine::default()
            .trans::<Inactive, _>(toggle_pressed, Active)
            .trans::<Active, _>(toggle_pressed, Inactive),
        PlayerAction::default_input_map(),
        ActionState::<PlayerAction>::default(),
        RigidBody::Dynamic,
        Collider::rectangle(40.0, 40.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
    ));
}
