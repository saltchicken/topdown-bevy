mod menu;
mod player;
mod state;

use bevy::prelude::*;
use menu::MenuPlugin;
use player::PlayerPlugin;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "topdown".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((MenuPlugin, PlayerPlugin))
        .run();
}
