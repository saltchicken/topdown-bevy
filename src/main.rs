use bevy::prelude::*;
use seldom_state::prelude::*;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "Physics Simulator Shell";

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Inactive;

#[derive(Clone, Copy, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Active;

fn space_pressed(In(_): In<Entity>, keys: Res<ButtonInput<KeyCode>>) -> bool {
    keys.just_pressed(KeyCode::Space)
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
        .add_plugins(StateMachinePlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, update_color)
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
            .trans::<Inactive, _>(space_pressed, Active)
            .trans::<Active, _>(space_pressed, Inactive),
    ));
}
