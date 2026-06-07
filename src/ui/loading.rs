use crate::core::state::GameState;
use crate::core::utils::despawn_screen;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::entities::enemy::EnemyConfig;
use crate::entities::map::MapConfig;
use crate::entities::player::PlayerConfig;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<GameAssets>(),
        )
        .add_systems(OnEnter(GameState::Loading), setup_loading_screen)
        // Add cleanup system to remove the UI when loading finishes
        .add_systems(OnExit(GameState::Loading), despawn_screen::<LoadingUI>);
    }
}

const BG_COLOR: Color = Color::srgb(0.05, 0.05, 0.05);
const FONT_SIZE: f32 = 60.0;

// Marker component so we can find and despawn the loading screen later
#[derive(Component)]
struct LoadingUI;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "data/level.map.ron")]
    pub map_config: Handle<MapConfig>,
    #[asset(path = "data/stats.player.ron")]
    pub player_config: Handle<PlayerConfig>,
    #[asset(path = "data/stats.enemy.ron")]
    pub enemy_config: Handle<EnemyConfig>,
    #[asset(path = "player_idle.png")]
    pub player_idle: Handle<Image>,
    #[asset(path = "player_walk.png")]
    pub player_walk: Handle<Image>,
    #[asset(path = "enemy_idle.png")]
    pub enemy_idle: Handle<Image>,
    #[asset(path = "enemy_walk.png")]
    pub enemy_walk: Handle<Image>,
    #[asset(path = "tiles.png")]
    pub tiles: Handle<Image>,
}

fn setup_loading_screen(mut commands: Commands) {
    // 1. Spawn Loading Screen UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BG_COLOR), // Dark background
            LoadingUI,                 // Tag it for cleanup
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
