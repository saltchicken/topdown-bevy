use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerConfig {
    pub size: f32,
    pub base_speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            size: 40.0,
            base_speed: 300.0,
        }
    }
}
