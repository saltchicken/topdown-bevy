use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<PauseState>()
            .configure_sets(
                Update,
                GameplaySet
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameplaySet;
