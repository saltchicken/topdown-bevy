use crate::core::state::GameState;
use crate::core::utils::despawn_screen;
use crate::render::z_layers::ZLayer;
use crate::ui::loading::GameAssets;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_map)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<MapEntity>);
    }
}

#[derive(Component)]
pub struct MapEntity;

fn spawn_map(mut commands: Commands, game_assets: Res<GameAssets>) {
    let map_data = include_str!("../../assets/data/map.txt");
    let lines: Vec<&str> = map_data.lines().filter(|l| !l.is_empty()).collect();
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
            
            let texture_index = match ch {
                '.' => 0, // Grass
                ',' => 1, // Dirt
                '~' => 2, // Water
                '#' => 3, // Stone
                _ => 0,
            };

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(texture_index),
                        ..Default::default()
                    },
                    MapEntity,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);

            if ch == '#' {
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
