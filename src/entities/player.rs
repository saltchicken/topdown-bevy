use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;
use self::states::{active::*, inactive::*};

pub mod states;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Toggle,
    Up,
    Down,
    Left,
    Right,
}

fn toggle_pressed(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    query.get(entity).is_ok_and(|action_state| action_state.just_pressed(&PlayerAction::Toggle))
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_plugins((ActivePlugin, InactivePlugin))
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
        InputMap::default()
            .with(PlayerAction::Toggle, KeyCode::Space)
            .with(PlayerAction::Up, KeyCode::KeyW)
            .with(PlayerAction::Down, KeyCode::KeyS)
            .with(PlayerAction::Left, KeyCode::KeyA)
            .with(PlayerAction::Right, KeyCode::KeyD),
        ActionState::<PlayerAction>::default(),
    ));
}
