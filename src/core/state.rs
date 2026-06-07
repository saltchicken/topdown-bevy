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
            )
            .configure_sets(
                FixedUpdate,
                GameplaySet
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PauseState::Running)),
            )
            .add_systems(Startup, pause_time)
            .add_systems(OnEnter(GameState::Playing), resume_time)
            .add_systems(OnExit(GameState::Playing), pause_time)
            .add_systems(OnEnter(PauseState::Paused), pause_time)
            .add_systems(OnExit(PauseState::Paused), resume_time);
    }
}

fn pause_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
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
