pub mod idle;
pub mod running;
pub mod walking;

use bevy::prelude::*;

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(idle::on_idle_enter)
            .add_observer(walking::on_walking_enter)
            .add_systems(FixedUpdate, walking::on_walking_update)
            .add_observer(running::on_running_enter)
            .add_systems(FixedUpdate, running::on_running_update);
    }
}
