mod loading;
mod menu;
mod player;
mod state;
mod utils;

use bevy::prelude::*;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use state::GameState;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "topdown";

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: WINDOW_TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((LoadingPlugin, MenuPlugin, PlayerPlugin))
        .run();
}
