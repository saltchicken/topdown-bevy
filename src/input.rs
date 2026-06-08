use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Toggle,
    Up,
    Down,
    Left,
    Right,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with(Self::Toggle, KeyCode::Space)
            .with(Self::Up, KeyCode::KeyW)
            .with(Self::Down, KeyCode::KeyS)
            .with(Self::Left, KeyCode::KeyA)
            .with(Self::Right, KeyCode::KeyD)
    }
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}
