use bevy::prelude::*;

use super::states::{idle::IdlePlugin, running::RunningPlugin, walking::WalkingPlugin};
use super::systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((IdlePlugin, RunningPlugin, WalkingPlugin))
            .add_systems(Startup, systems::setup_player);
    }
}
