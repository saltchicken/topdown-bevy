use crate::core::state::GameState;
use crate::core::utils::despawn_screen;
use crate::render::z_layers::ZLayer;
use crate::ui::loading::GameAssets;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Asset, TypePath, Deserialize, Clone)]
pub struct MapConfig {
    pub tile_mapping: HashMap<char, TileData>,
    pub layout: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct TileData {
    pub texture_index: u32,
    pub is_rigid_body: bool,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_common_assets::ron::RonAssetPlugin::<MapConfig>::new(&["map.ron"]))
            .add_systems(OnEnter(GameState::Playing), spawn_map)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<MapEntity>);
    }
}

#[derive(Component)]
pub struct MapEntity;

fn spawn_map(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    map_configs: Res<Assets<MapConfig>>,
) {
    let config = map_configs.get(&game_assets.map_config).expect("Map config should be loaded");
    let lines = &config.layout;
    let size_y = lines.len() as u32;
    let size_x = if size_y > 0 { lines[0].len() as u32 } else { 0 };

    let tilemap_size = TilemapSize { x: size_x, y: size_y };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    let tilemap_entity = commands.spawn(MapEntity).id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for (row, line) in lines.iter().enumerate() {
        let y = size_y - 1 - row as u32; // Reverse Y so it draws top-down properly
        for (col, ch) in line.chars().enumerate() {
            let x = col as u32;
            let tile_pos = TilePos { x, y };
            
            let default_tile = TileData { texture_index: 0, is_rigid_body: false };
            let tile_data = config.tile_mapping.get(&ch).unwrap_or(&default_tile);

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile_data.texture_index),
                        ..Default::default()
                    },
                    MapEntity,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);

            if tile_data.is_rigid_body {
                let collider_entity = commands.spawn((
                    Collider::rectangle(tile_size.x, tile_size.y),
                    RigidBody::Static,
                    Friction::new(0.0),
                    Restitution::new(0.0),
                    Transform::from_xyz(
                        x as f32 * tile_size.x,
                        y as f32 * tile_size.y,
                        0.0,
                    ),
                )).id();
                commands.entity(tilemap_entity).add_child(collider_entity);
            }
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(game_assets.tiles.clone()),
        tile_size,
        transform: Transform::from_xyz(
            -(tilemap_size.x as f32 * tile_size.x) / 2.0,
            -(tilemap_size.y as f32 * tile_size.y) / 2.0,
            ZLayer::Background.to_f32(),
        ),
        ..Default::default()
    });
}
