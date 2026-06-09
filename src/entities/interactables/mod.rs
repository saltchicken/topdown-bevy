pub mod coin;
pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use events::InteractionEvent;
use systems::detect_interactions;

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InteractionEvent>()
            .add_systems(Update, detect_interactions)
            .add_plugins(coin::CoinPlugin);
    }
}
