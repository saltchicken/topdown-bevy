pub mod entities;
pub mod input;
pub mod effects;
pub mod physics;
pub mod ui;
pub mod world;

use avian2d::prelude::*;
use bevy::prelude::*;
use entities::enemy::EnemyPlugin;
use entities::interactables::InteractablesPlugin;
use entities::player::PlayerPlugin;
use seldom_state::prelude::*;
use world::WorldPlugin;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const WINDOW_TITLE: &str = "Hello World";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: WINDOW_TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(StateMachinePlugin::default())
        .add_plugins(input::GameInputPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(InteractablesPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(effects::EffectsPlugin)
        .add_plugins(ui::UiPlugin)
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        effects::CameraShake {
            intensity: 0.0,
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        },
    ));
}
