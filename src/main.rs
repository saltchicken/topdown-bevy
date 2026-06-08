use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "Physics Simulator Shell";

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Toggle,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Inactive;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Active;

fn toggle_pressed(In(entity): In<Entity>, query: Query<&ActionState<PlayerAction>>) -> bool {
    query.get(entity).is_ok_and(|action_state| action_state.just_pressed(&PlayerAction::Toggle))
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<PlayerAction>)>,
) {
    for (mut transform, action_state) in &mut query {
        let mut direction = Vec2::ZERO;
        if action_state.pressed(&PlayerAction::Up) {
            direction.y += 1.0;
        }
        if action_state.pressed(&PlayerAction::Down) {
            direction.y -= 1.0;
        }
        if action_state.pressed(&PlayerAction::Left) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&PlayerAction::Right) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            transform.translation += direction.normalize().extend(0.0) * 300.0 * time.delta_secs();
        }
    }
}

fn update_color(
    mut query: Query<
        (&mut Sprite, Has<Active>),
        Or<(Added<Active>, Added<Inactive>)>,
    >,
) {
    for (mut sprite, is_active) in &mut query {
        if is_active {
            sprite.color = Color::srgb(1.0, 0.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.0, 1.0, 0.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                title: WINDOW_TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(StateMachinePlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_color, player_movement))
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

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
