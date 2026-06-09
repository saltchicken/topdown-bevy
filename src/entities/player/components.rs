use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Speed(pub f32);
