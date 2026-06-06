use bevy::prelude::*;
use crate::state::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(Update, check_loading.run_if(in_state(GameState::Loading)))
            // Add cleanup system to remove the UI when loading finishes
            .add_systems(OnExit(GameState::Loading), cleanup_loading);
    }
}

// Marker component so we can find and despawn the loading screen later
#[derive(Component)]
struct LoadingUI;

#[derive(Resource)]
pub struct GameAssets {
    pub player_idle: Handle<Image>,
    pub player_walk: Handle<Image>,
    pub player_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct MinimumLoadTimer(Timer);

fn start_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
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

    // 2. Load Assets
    let player_idle = asset_server.load("player_idle.png");
    let player_walk = asset_server.load("player_walk.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 4, None, None);
    let player_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets {
        player_idle,
        player_walk,
        player_layout,
    });

    // Add a 2-second artificial delay for testing
    commands.insert_resource(MinimumLoadTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn check_loading(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut delay_timer: ResMut<MinimumLoadTimer>,
) {
    // Progress the timer
    delay_timer.0.tick(time.delta());

    let idle_loaded = asset_server.is_loaded_with_dependencies(&game_assets.player_idle);
    let walk_loaded = asset_server.is_loaded_with_dependencies(&game_assets.player_walk);

    // Transition only if assets are loaded AND the timer has finished
    if idle_loaded && walk_loaded && delay_timer.0.finished() {
        next_state.set(GameState::MainMenu);
    }
}

// Despawns the loading screen layout when the state changes
fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
