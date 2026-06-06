use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::state::GameState;
use crate::utils::despawn_screen;

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

// Marker component so we can find and despawn the loading screen later
#[derive(Component)]
struct LoadingUI;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "player_idle.png")]
    pub player_idle: Handle<Image>,
    #[asset(path = "player_walk.png")]
    pub player_walk: Handle<Image>,
}

fn setup_loading_screen(mut commands: Commands) {
    // 1. Spawn Loading Screen UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.05, 0.05, 0.05).into(), // Dark background
                ..default()
            },
            LoadingUI, // Tag it for cleanup
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

