use bevy::prelude::*;
use crate::state::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), start_loading)
            .add_systems(Update, check_loading.run_if(in_state(GameState::Loading)));
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub player_idle: Handle<Image>,
    pub player_walk: Handle<Image>,
    pub player_layout: Handle<TextureAtlasLayout>,
}

fn start_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_idle = asset_server.load("player_idle.png");
    let player_walk = asset_server.load("player_walk.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 4, None, None);
    let player_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets {
        player_idle,
        player_walk,
        player_layout,
    });
}

fn check_loading(
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let idle_loaded = asset_server.is_loaded_with_dependencies(&game_assets.player_idle);
    let walk_loaded = asset_server.is_loaded_with_dependencies(&game_assets.player_walk);

    if idle_loaded && walk_loaded {
        next_state.set(GameState::MainMenu);
    }
}
