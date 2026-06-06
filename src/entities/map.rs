use crate::core::state::GameState;
use crate::core::utils::despawn_screen;
use crate::render::z_layers;
use crate::ui::loading::GameAssets;
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
    let tilemap_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    let tilemap_entity = commands.spawn(MapEntity).id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                    MapEntity,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
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
            z_layers::BACKGROUND,
        ),
        ..Default::default()
    });
}
