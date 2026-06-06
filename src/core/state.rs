use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}
