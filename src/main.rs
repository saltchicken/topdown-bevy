use avian2d::prelude::*;
use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "Physics Simulator Shell";

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
        .add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 98.1))
        .add_systems(Startup, setup_physics_scene)
        .run();
}

fn setup_physics_scene(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Static Ground Environment
    commands.spawn((
        Transform::from_xyz(0.0, -100.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(800.0, 20.0),
    ));

    // Dynamic Physics Object
    commands.spawn((
        Transform::from_xyz(0.0, 200.0, 0.0),
        RigidBody::Dynamic,
        Collider::circle(20.0),
        Restitution::new(0.8),
    ));
}
