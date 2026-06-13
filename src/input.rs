use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Dash,
    Walk,
    Interact, // Mapped to E for active interactions
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MovementIntention {
    Idle,
    Walking,
    Running,
    Dashing,
}

impl MovementIntention {
    pub fn evaluate(action_state: &ActionState<PlayerAction>) -> Self {
        if action_state.axis_pair(&PlayerAction::Move).length_squared() == 0.0 {
            Self::Idle
        } else if action_state.just_pressed(&PlayerAction::Dash) {
            Self::Dashing
        } else if action_state.pressed(&PlayerAction::Walk) {
            Self::Walking
        } else {
            Self::Running
        }
    }
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Move, VirtualDPad::wasd())
            .with(Self::Dash, KeyCode::ShiftLeft)
            .with(Self::Walk, KeyCode::ControlLeft)
            .with(Self::Interact, KeyCode::KeyE)
    }
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}
