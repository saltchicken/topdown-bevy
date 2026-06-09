use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Coin {
    pub value: u32,
}

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_coins);
    }
}

fn setup_coins(mut commands: Commands) {
    // Spawning a test coin
    commands.spawn((
        Coin { value: 1 },
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        Transform::from_xyz(150.0, 100.0, 0.0),
        Collider::circle(10.0),
        Sensor,
    ));
}
