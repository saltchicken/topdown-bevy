use crate::render::z_layers::{ZLayer, Y_SORT_MULTIPLIER};
use bevy::prelude::*;

pub struct YSortPlugin;

impl Plugin for YSortPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, y_sort_system);
    }
}

#[derive(Component)]
pub struct YSort(pub ZLayer);

fn y_sort_system(mut query: Query<(&YSort, &mut Transform)>) {
    for (y_sort_base, mut transform) in &mut query {
        let new_z = y_sort_base.0.to_f32() - transform.translation.y * Y_SORT_MULTIPLIER;

        // Only mutate the transform if the Z value actually changed.
        // We use an epsilon check for floating-point safety.
        if (transform.translation.z - new_z).abs() > f32::EPSILON {
            transform.translation.z = new_z;
        }
    }
}
