use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Interactable;

// Add this so any entity can be marked capable of interacting
#[derive(Component, Reflect, Default)]
pub struct Interactor;
