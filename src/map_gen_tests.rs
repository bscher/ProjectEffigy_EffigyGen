#[allow(unused_imports)]
use effigy_shared::logging::{debug, error, info, trace, warn};

use effigy_shared::game::map::{MapChunkXY, MapGrid, MAP_CHUNK_TILES_LENGTH};

#[cfg(feature = "images")]
use super::images::map_as_image::save_map_grid_as_image;
use super::map_gen::MapGenerator;

#[test]
fn generate_example_tile_grid() {
    let seed: u32 = 453537;
    let map_chunks_size = MapChunkXY { x: 2, y: 2 };
    let grid: MapGrid = MapGenerator::new(seed, &map_chunks_size)
        .with_terrain(0.2, 0.5)
        .with_buildings(0.1, 0.25)
        .with_water(0.2, 0.5)
        .with_trees(0.2, 0.5)
        .generate();

    assert_eq!(grid.get_chunks_bounds(), map_chunks_size);
    assert_eq!(
        grid.get_chunks_count(),
        (map_chunks_size.x as usize) * (map_chunks_size.y as usize)
    );

    assert_eq!(
        grid.get_tiles_bounds().x,
        map_chunks_size.x * MAP_CHUNK_TILES_LENGTH
    );
    assert_eq!(
        grid.get_tiles_bounds().y,
        map_chunks_size.y * MAP_CHUNK_TILES_LENGTH
    );
    assert_eq!(
        grid.get_tiles_count(),
        (grid.get_tiles_bounds().x as usize) * (grid.get_tiles_bounds().y as usize)
    );

    #[cfg(feature = "images")]
    save_map_grid_as_image(&grid, &format!("map_gen_{}.png", seed)).unwrap();
}
