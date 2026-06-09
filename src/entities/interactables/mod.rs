pub mod coin;

use bevy::prelude::*;

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(coin::CoinPlugin);
    }
}
