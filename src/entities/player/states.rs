pub mod idle;
pub mod dashing;
pub mod running;

use bevy::prelude::*;

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(idle::on_enter)
            .add_observer(running::on_enter)
            .add_systems(FixedUpdate, running::on_update)
            .add_observer(dashing::on_enter)
            .add_systems(FixedUpdate, dashing::on_update);
    }
}
