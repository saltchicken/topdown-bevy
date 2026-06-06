use crate::core::state::GameplaySet;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow.in_set(GameplaySet));
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow {
    pub target: Entity,
    pub decay_rate: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn camera_follow(
    mut camera_query: Query<(&mut Transform, &CameraFollow), With<MainCamera>>,
    transform_query: Query<&Transform, Without<MainCamera>>,
    time: Res<Time>,
) {
    let Ok((mut camera_transform, camera_follow)) = camera_query.single_mut() else {
        return;
    };

    let Ok(target_transform) = transform_query.get(camera_follow.target) else {
        return;
    };

    let target_pos = target_transform.translation;

    // Preserve the camera's Z position while setting the XY target
    let target = target_pos.truncate().extend(camera_transform.translation.z);

    camera_transform.translation = camera_transform
        .translation
        .lerp(target, 1.0 - f32::exp(-camera_follow.decay_rate * time.delta_secs()));
}
