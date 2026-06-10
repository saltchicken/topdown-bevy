use bevy::prelude::*;

#[derive(Message)]
pub struct SpawnRequest<T> {
    pub position: Vec2,
    pub payload: T,
}
