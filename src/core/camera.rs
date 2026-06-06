use bevy::prelude::*;
use crate::entities::player::Player;
use crate::core::state::GameplaySet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow.in_set(GameplaySet));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let target_pos = player_transform.translation;
    
    // Preserve the camera's Z position while setting the XY target
    let target = target_pos.truncate().extend(camera_transform.translation.z);
    
    let decay_rate = 2.0; 
    camera_transform.translation = camera_transform.translation.lerp(
        target, 
        1.0 - f32::exp(-decay_rate * time.delta_secs())
    );   
}
