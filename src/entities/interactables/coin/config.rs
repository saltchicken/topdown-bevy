use bevy::prelude::*;

#[derive(Resource)]
pub struct CoinConfig {
    pub default_value: u32,
    pub size: f32,
    pub collider_radius: f32,
    pub color: Color,
}

impl Default for CoinConfig {
    fn default() -> Self {
        Self {
            default_value: 1,
            size: 20.0,
            collider_radius: 10.0,
            color: Color::srgb(1.0, 1.0, 0.0),
        }
    }
}
