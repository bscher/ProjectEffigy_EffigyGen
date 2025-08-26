#[allow(unused_imports)]
use effigy_shared::logging::{debug, error, info, trace, warn};

use effigy_shared::game::map::{
    MapChunkXY, MapGrid, MapTileIndex, MapTileVisualType, MapTileXY, SquareBounds,
};

use super::noise_gen::{
    generate_buildings_noise, generate_foilage_noise, generate_landscape_noise,
    generate_water_noise,
};

pub struct MapGenerator {
    seed: u32,
    map_grid: MapGrid,
}
impl MapGenerator {
    pub fn new(seed: u32, map_chunks_count: &MapChunkXY) -> Self {
        MapGenerator {
            seed,
            map_grid: MapGrid::new(map_chunks_count),
        }
    }

    /// On any land tiles, generate a map containing tree tiles
    pub fn with_trees(mut self, density: f64, scatter: f64) -> Self {
        assert!(
            density >= 0.0 && density <= 1.0,
            "Density value needs to be between 0.0 and 1.0 (inclusive)"
        );
        assert!(
            scatter >= 0.0 && scatter <= 1.0,
            "Scatter value needs to be between 0.0 and 1.0 (inclusive)"
        );

        let start_time = {
            info!(
                "Populating map with trees: density={} and scatter={} ...",
                density, scatter
            );
            std::time::Instant::now()
        };

        let seed = self.seed.wrapping_add(3453453666);
        let width = self.map_grid.get_tiles_bounds().x as usize;
        let height = self.map_grid.get_tiles_bounds().y as usize;

        let mut noise_map = generate_foilage_noise(seed, width, height, scatter);

        info!(
            "... generated tree noise map in {:.3} seconds ...",
            start_time.elapsed().as_secs_f32()
        );

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_trees.png");

        let noise_max = 0.0 - (0.5 + (0.5 * (1.0 - density)));
        for y in 0..height {
            for x in 0..width {
                if noise_map.get_value(x, y) < noise_max {
                    noise_map.set_value(x, y, 1.0);
                    let tile = self.map_grid
                        .get_tile_mut(&MapTileXY {
                            x: x as MapTileIndex,
                            y: y as MapTileIndex,
                        })
                        .expect(
                            format!(
                                "Expected tile in bounds when adding tree at ({}, {}) with map size ({}, {})",
                                x, y, width, height
                            )
                            .as_str(),
                        );
                    if tile.visual_type == MapTileVisualType::Land {
                        tile.set_visual_type(&MapTileVisualType::LandTree)
                            .set_walkability_blocked(&SquareBounds::ALL);
                    }
                } else {
                    noise_map.set_value(x, y, -1.0);
                }
            }
        }

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_trees_mask.png");

        info!(
            "... done populating map with trees! Completed in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
        self
    }

    /// On any land tiles, generate a map containing water tiles
    pub fn with_water(mut self, density: f64, scatter: f64) -> Self {
        assert!(
            density >= 0.0 && density <= 1.0,
            "Density value needs to be between 0.0 and 1.0 (inclusive)"
        );
        assert!(
            scatter >= 0.0 && scatter <= 1.0,
            "Scatter value needs to be between 0.0 and 1.0 (inclusive)"
        );

        let seed = self.seed.wrapping_add(16654312);
        let width = self.map_grid.get_tiles_bounds().x as usize;
        let height = self.map_grid.get_tiles_bounds().y as usize;

        let start_time = {
            info!(
                "Populating map with water: density={} scatter={} ...",
                density, scatter
            );
            std::time::Instant::now()
        };

        let mut noise_map = generate_water_noise(seed, width, height, scatter);

        info!(
            "... generated water noise map in {:.3} seconds ...",
            start_time.elapsed().as_secs_f32()
        );

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_water.png");

        let noise_max = -1.0 + (0.8 * density);
        for y in 0..height {
            for x in 0..width {
                if noise_map.get_value(x, y) < noise_max {
                    noise_map.set_value(x, y, 1.0);
                    let tile = self
                        .map_grid
                        .get_tile_mut(&MapTileXY {
                            x: x as MapTileIndex,
                            y: y as MapTileIndex,
                        })
                        .unwrap();
                    if tile.visual_type == MapTileVisualType::Land {
                        tile.set_visual_type(&MapTileVisualType::Water)
                            .set_walkability_blocked(&SquareBounds::ALL);
                    }
                } else {
                    noise_map.set_value(x, y, -1.0);
                }
            }
        }

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_water_mask.png");

        info!(
            "... done populating map with water! Completed in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
        self
    }

    /// On any land tiles, generate a map containing terrain tiles
    pub fn with_terrain(mut self, density: f64, scatter: f64) -> Self {
        assert!(
            density >= 0.0 && density <= 1.0,
            "Density value needs to be between 0.0 and 1.0 (inclusive)"
        );
        assert!(
            scatter >= 0.0 && scatter <= 1.0,
            "Scatter value needs to be between 0.0 and 1.0 (inclusive)"
        );

        let seed = self.seed.wrapping_add(1443443);
        let width = self.map_grid.get_tiles_bounds().x as usize;
        let height = self.map_grid.get_tiles_bounds().y as usize;

        let start_time = {
            info!(
                "Populating map with terrain: density={} and scatter={} ...",
                density, scatter
            );
            std::time::Instant::now()
        };

        let mut noise_map = generate_landscape_noise(seed, width, height, scatter);

        info!(
            "... generated terrain noise map in {:.3} seconds ...",
            start_time.elapsed().as_secs_f32()
        );

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_terrain.png");

        let noise_max = -1.0 + (0.8 * density);
        for y in 0..height {
            for x in 0..width {
                if noise_map.get_value(x, y) < noise_max {
                    noise_map.set_value(x, y, 1.0);
                    let tile = self
                        .map_grid
                        .get_tile_mut(&MapTileXY {
                            x: x as MapTileIndex,
                            y: y as MapTileIndex,
                        })
                        .unwrap();
                    // Terrain overrides everything, excluding water
                    if tile.visual_type == MapTileVisualType::Land {
                        tile.set_visual_type(&MapTileVisualType::Terrain)
                            .set_walkability_blocked(&SquareBounds::ALL);
                    }
                } else {
                    noise_map.set_value(x, y, -1.0);
                }
            }
        }

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_terrain_mask.png");

        info!(
            "... done populating map with terrain! Completed in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
        self
    }

    pub fn with_buildings(mut self, density: f64, scale: f64) -> Self {
        assert!(
            density >= 0.0 && density <= 1.0,
            "Density value needs to be between 0.0 and 1.0 (inclusive)"
        );
        assert!(
            scale > 0.0 && scale <= 1.0,
            "Scale value needs to be between 0.0 (exclusive) and 1.0 (inclusive)"
        );

        let seed = self.seed.wrapping_add(2344443);
        let width = self.map_grid.get_tiles_bounds().x as usize;
        let height = self.map_grid.get_tiles_bounds().y as usize;

        let start_time = {
            info!("Populating map with buildings: density={} ...", density);
            std::time::Instant::now()
        };

        let mut noise_map = generate_buildings_noise(seed, width, height, scale);

        info!(
            "... generated buildings noise map in {:.3} seconds ...",
            start_time.elapsed().as_secs_f32()
        );

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_buildings.png");

        let noise_max = -1.0 + (0.1 * density);
        for y in 0..height {
            for x in 0..width {
                if noise_map.get_value(x, y) < noise_max {
                    noise_map.set_value(x, y, 1.0);
                    let tile = self
                        .map_grid
                        .get_tile_mut(&MapTileXY {
                            x: x as MapTileIndex,
                            y: y as MapTileIndex,
                        })
                        .unwrap();
                    if tile.visual_type == MapTileVisualType::Land {
                        tile.set_visual_type(&MapTileVisualType::Building)
                            .set_walkability_blocked(&SquareBounds::ALL);
                    }
                } else {
                    noise_map.set_value(x, y, -1.0);
                }
            }
        }

        #[cfg(feature = "images")]
        noise_map.write_to_file("generated_buildings_mask.png");

        info!(
            "... done populating map with buildings! Completed in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
        self
    }

    /// Cleans up and returns the finalized map grid
    pub fn generate(mut self) -> MapGrid {
        let start_time = {
            info!("Generating final map ...");
            std::time::Instant::now()
        };

        simplify_map_walkability_blockage(&mut self.map_grid);

        // ...

        info!(
            "... done finalizing map! Completed in {:.3} seconds",
            start_time.elapsed().as_secs_f32()
        );
        self.map_grid
    }
}

fn simplify_map_walkability_blockage(map_grid: &mut MapGrid) {
    info!("Simplifying walkability blockage between tiles ...");
    let func_start_time = std::time::Instant::now();

    let tile_width = map_grid.get_tiles_bounds().x as usize;
    let tile_height = map_grid.get_tiles_bounds().y as usize;

    for y in 0..tile_height {
        for x in 0..tile_width {
            let mut current_tile_copy = map_grid
                .get_tile(&MapTileXY {
                    x: x as MapTileIndex,
                    y: y as MapTileIndex,
                })
                .unwrap()
                .clone();
            // Check tile (x - 1)
            if x > 0 {
                // If the same tile type to the left shares a border, remove the blockage
                if current_tile_copy.walkability_blocked.left {
                    let left_tile = map_grid
                        .get_tile_mut(&MapTileXY {
                            x: (x - 1) as MapTileIndex,
                            y: y as MapTileIndex,
                        })
                        .unwrap();
                    if left_tile.visual_type == current_tile_copy.visual_type
                        && left_tile.walkability_blocked.right
                    {
                        current_tile_copy.walkability_blocked.left = false;
                        left_tile.walkability_blocked.right = false;
                    }
                }
            } else {
                // Left edge of map is always blocked
                current_tile_copy.walkability_blocked.left = true;
            }
            // Check tile (y - 1)
            if y > 0 {
                // If the same tile type downwards shares a border, remove the blockage
                if current_tile_copy.walkability_blocked.down {
                    let down_tile = map_grid
                        .get_tile_mut(&MapTileXY {
                            x: x as MapTileIndex,
                            y: (y - 1) as MapTileIndex,
                        })
                        .unwrap();
                    if down_tile.visual_type == current_tile_copy.visual_type
                        && down_tile.walkability_blocked.up
                    {
                        current_tile_copy.walkability_blocked.down = false;
                        down_tile.walkability_blocked.up = false;
                    }
                }
            } else {
                // Bottom edge of map is always blocked
                current_tile_copy.walkability_blocked.down = true;
            }
            // Check tile (width - 1)
            if x < tile_width - 1 {
                // If the same tile type to the right shares a border, remove the blockage
                if current_tile_copy.walkability_blocked.right {
                    let right_tile = map_grid
                        .get_tile_mut(&MapTileXY {
                            x: (x + 1) as MapTileIndex,
                            y: y as MapTileIndex,
                        })
                        .unwrap();
                    if right_tile.visual_type == current_tile_copy.visual_type
                        && right_tile.walkability_blocked.left
                    {
                        current_tile_copy.walkability_blocked.right = false;
                        right_tile.walkability_blocked.left = false;
                    }
                }
            } else {
                // Right edge of map is always blocked
                current_tile_copy.walkability_blocked.right = true;
            }
            // Check tile (height - 1)
            if y < tile_height - 1 {
                // If the same tile type upwards shares a border, remove the blockage
                if current_tile_copy.walkability_blocked.up {
                    let up_tile = map_grid
                        .get_tile_mut(&MapTileXY {
                            x: x as MapTileIndex,
                            y: (y + 1) as MapTileIndex,
                        })
                        .unwrap();
                    if up_tile.visual_type == current_tile_copy.visual_type
                        && up_tile.walkability_blocked.down
                    {
                        current_tile_copy.walkability_blocked.up = false;
                        up_tile.walkability_blocked.down = false;
                    }
                }
            } else {
                // Top edge of map is always blocked
                current_tile_copy.walkability_blocked.up = true;
            }
            map_grid
                .get_tile_mut(&MapTileXY {
                    x: x as MapTileIndex,
                    y: y as MapTileIndex,
                })
                .unwrap()
                .set_walkability_blocked(&current_tile_copy.walkability_blocked);
        }
    }

    info!(
        "... done simplifying walkability blockage between tiles! Completed in {:.3} seconds",
        func_start_time.elapsed().as_secs_f32()
    );
}
