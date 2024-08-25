use crate::game::assets::HandleMap;
use crate::game::assets::ImageKey;
use crate::game::town::Town;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
// use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TilemapPlugin);
    app.observe(spawn_random_tilemap);
}

#[derive(Event, Debug)]
pub struct SpawnRandomTilemap {
    pub town: Town,
}

fn spawn_random_tilemap(
    trigger: Trigger<SpawnRandomTilemap>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    // let mut _rng = rand::thread_rng();
    let evt = trigger.event();
    let town = &evt.town;
    let x = town.width;
    let y = town.height;

    let texture_handle: Handle<Image> = image_handles.get(&ImageKey::Tiles).unwrap().clone();

    let tilemap_entity = commands.spawn_empty().id();
    let map_size = TilemapSize { x, y };
    let mut tile_storage = TileStorage::empty(map_size);
    for x in 0..x {
        for y in 0..y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: town.sample_tile(tile_pos),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
