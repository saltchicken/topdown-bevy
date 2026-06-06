mod loading;
mod menu;
mod player;
mod state;

use bevy::prelude::*;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use state::GameState;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "topdown".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((LoadingPlugin, MenuPlugin, PlayerPlugin))
        .run();
}
