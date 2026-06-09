use bevy::prelude::*;

use super::states::{active::ActivePlugin, inactive::InactivePlugin};
use super::systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ActivePlugin, InactivePlugin))
            .add_systems(Startup, systems::setup_player);
    }
}
