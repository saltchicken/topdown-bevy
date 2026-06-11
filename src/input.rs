use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Dash,
    Walk,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Move, VirtualDPad::wasd())
            .with(Self::Dash, KeyCode::ShiftLeft)
            .with(Self::Walk, KeyCode::ControlLeft)
    }
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}
