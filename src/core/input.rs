use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GameAction>::default())
            .init_resource::<ActionState<GameAction>>()
            .insert_resource(GameAction::default_input_map());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum GameAction {
    #[actionlike(DualAxis)]
    Move,
    TogglePause,
}

impl GameAction {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Move, VirtualDPad::wasd())
            .with_dual_axis(Self::Move, VirtualDPad::arrow_keys())
            .with(Self::TogglePause, KeyCode::Escape)
    }
}
