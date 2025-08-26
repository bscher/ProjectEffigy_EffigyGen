use std::path::{Path, PathBuf};
use std::{fs, io};

use image::{self, ColorType};

use effigy_shared::game::map::{MapGrid, MapTile, MapTileVisualType, MapTileXY};

const BYTES_PER_TILE_PIXEL: usize = 3;
type Rgb8 = [u8; BYTES_PER_TILE_PIXEL];
const TILE_PIXEL_COLOR_TYPE: ColorType = ColorType::Rgb8;

const fn get_rgb8_from_map_tile(tile: &MapTile) -> Rgb8 {
    get_rgb8_from_map_visual_type(&tile.visual_type)
}

const fn get_rgb8_from_map_visual_type(visual_type: &MapTileVisualType) -> Rgb8 {
    match visual_type {
        MapTileVisualType::Land => [0, 0, 0],
        MapTileVisualType::LandTree => [0, 255, 0],
        MapTileVisualType::Water => [0, 0, 255],
        MapTileVisualType::Terrain => [255, 255, 255],
        MapTileVisualType::Building => [255, 0, 0],
    }
}

pub fn save_map_grid_as_image(grid: &MapGrid, image_file_name: &str) -> Result<PathBuf, io::Error> {
    let target_dir = Path::new("example_images/");

    // Create the target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir(target_dir)?;
    }

    let image_bytes: Vec<u8> = {
        let mut bytes = Vec::with_capacity(grid.get_tiles_count() * BYTES_PER_TILE_PIXEL);
        for y in 0..grid.get_tiles_bounds().y {
            for x in 0..grid.get_tiles_bounds().x {
                let tile = grid.get_tile(&MapTileXY { x, y }).unwrap();
                let (r, g, b) = {
                    let mut tile_rgb = get_rgb8_from_map_tile(tile);
                    if tile.visual_type != MapTileVisualType::Land
                        && !tile.walkability_blocked.is_any()
                    {
                        tile_rgb[0] = tile_rgb[0] / 3 * 2;
                        tile_rgb[1] = tile_rgb[1] / 3 * 2;
                        tile_rgb[2] = tile_rgb[2] / 3 * 2;
                    }
                    (tile_rgb[0], tile_rgb[1], tile_rgb[2])
                };
                bytes.push(r);
                bytes.push(g);
                bytes.push(b);
            }
        }
        bytes
    };
    assert_eq!(
        image_bytes.len(),
        grid.get_tiles_count() * BYTES_PER_TILE_PIXEL,
        "Expected number of bytes to equal total grid tiles times the number of bytes per pixel"
    );

    let image_path = target_dir.join(image_file_name).with_extension("png");
    match image::save_buffer(
        &image_path,
        &*image_bytes,
        grid.get_tiles_bounds().x,
        grid.get_tiles_bounds().y as u32,
        TILE_PIXEL_COLOR_TYPE,
    ) {
        Ok(()) => {}
        Err(err) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Failed to save image buffer to file: {:?}, error: {:?}",
                    image_path, err
                ),
            ));
        }
    }

    Ok(image_path)
}
