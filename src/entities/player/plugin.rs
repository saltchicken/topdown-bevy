use bevy::prelude::*;

use super::states::{idle::IdlePlugin, running::RunningPlugin, walking::WalkingPlugin};
use super::systems;
use super::config::PlayerConfig;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_plugins((IdlePlugin, RunningPlugin, WalkingPlugin))
            .add_systems(Startup, systems::setup_player);
    }
}
