pub mod idle;
pub mod running;
pub mod walking;

use bevy::prelude::*;

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(idle::on_enter)
            .add_observer(walking::on_enter)
            .add_systems(FixedUpdate, walking::on_update)
            .add_observer(running::on_enter)
            .add_systems(FixedUpdate, running::on_update);
    }
}
