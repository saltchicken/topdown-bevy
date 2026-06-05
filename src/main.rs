mod menu;
mod player;
mod state;

use bevy::prelude::*;
use menu::MenuPlugin;
use player::PlayerPlugin;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((MenuPlugin, PlayerPlugin))
        .run();
}
