pub mod chase;
pub mod idle;
pub mod patrol;

use bevy::prelude::*;

pub struct EnemyStatePlugin;

impl Plugin for EnemyStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(idle::on_enter)
            .add_systems(FixedUpdate, idle::on_update)
            .add_observer(patrol::on_enter)
            .add_systems(FixedUpdate, patrol::on_update)
            .add_observer(chase::on_enter)
            .add_systems(FixedUpdate, chase::on_update);
    }
}
