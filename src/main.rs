use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "Hello World";

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
        .add_systems(Startup, setup_scene)
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
    ));
}
