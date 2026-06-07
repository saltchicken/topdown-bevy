use avian2d::prelude::*;
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "Physics Simulator Shell";

// --- 1. Define your Custom Material ---
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        // This path is relative to the `assets/` folder
        "shaders/custom_material.wgsl".into()
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
        .add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            // --- 2. Register the Material Plugin ---
            Material2dPlugin::<CustomMaterial>::default(), 
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 98.1))
        .add_systems(Startup, setup_physics_scene)
        .run();
}

fn setup_physics_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2d);

    // Static Ground Environment
    commands.spawn((
        Transform::from_xyz(0.0, -100.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(800.0, 20.0),
    ));

    // --- 3. Spawn a Mesh2d with your Custom Material ---
    // Note: The Mesh size and Collider size should match.
    commands.spawn((
        // Visuals
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: LinearRgba::new(0.0, 1.0, 0.5, 1.0), // A neat greenish tint
        })),
        // Physics
        Transform::from_xyz(0.0, 200.0, 0.0),
        RigidBody::Dynamic,
        Collider::circle(20.0),
        Restitution::new(0.8),
    ));
}
