#[allow(unused_imports)]
use effigy_shared::logging::{debug, error, info, trace, warn};

use noise::core::worley::distance_functions::euclidean;
use noise::core::worley::ReturnType;
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{MultiFractal, Perlin, RidgedMulti, Seedable, Worley};

use effigy_shared::game::map::MAP_CHUNK_TILES_LENGTH;

pub fn generate_foilage_noise(seed: u32, width: usize, height: usize, scatter: f64) -> NoiseMap {
    assert!(
        scatter >= 0.0 && scatter <= 1.0,
        "Scatter value needs to be between 0.0 and 1.0 (inclusive)"
    );
    assert!(
        width > 0 && width % (MAP_CHUNK_TILES_LENGTH as usize) == 0,
        "Width of map needs to be greater than 0 and be a multiple of the chunk side length"
    );

    let chunks_width: usize = width / (MAP_CHUNK_TILES_LENGTH as usize);
    let xy_radius: f64 = 0.125 * (chunks_width as f64);

    let ridged_multi_perlin = RidgedMulti::<Perlin>::default()
        .set_seed(seed)
        .set_frequency(80.0 * scatter);

    let noise_map = PlaneMapBuilder::<_, 2>::new(ridged_multi_perlin)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build();

    noise_map
}

pub fn generate_water_noise(seed: u32, width: usize, height: usize, scatter: f64) -> NoiseMap {
    assert!(
        scatter >= 0.0 && scatter <= 1.0,
        "Scatter value needs to be between 0.0 and 1.0 (inclusive)"
    );
    assert!(
        width > 0 && width % (MAP_CHUNK_TILES_LENGTH as usize) == 0,
        "Width of map needs to be greater than 0 and be a multiple of the chunk side length"
    );

    let chunks_width: usize = width / (MAP_CHUNK_TILES_LENGTH as usize);
    let xy_radius: f64 = 0.1 * (chunks_width as f64);

    let ridged_multi_perlin = RidgedMulti::<Perlin>::default()
        .set_seed(seed)
        .set_frequency(20.0 * scatter);

    let noise_map = PlaneMapBuilder::<_, 2>::new(ridged_multi_perlin)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build();

    noise_map
}

pub fn generate_landscape_noise(seed: u32, width: usize, height: usize, scatter: f64) -> NoiseMap {
    assert!(
        scatter >= 0.0 && scatter <= 1.0,
        "Scatter value needs to be between 0.0 and 1.0 (inclusive)"
    );
    assert!(
        width > 0 && width % (MAP_CHUNK_TILES_LENGTH as usize) == 0,
        "Width of map needs to be greater than 0 and be a multiple of the chunk side length"
    );

    let chunks_width: usize = width / (MAP_CHUNK_TILES_LENGTH as usize);
    let xy_radius: f64 = 0.5 * (chunks_width as f64);

    let ridged_multi_perlin = RidgedMulti::<Perlin>::default()
        .set_seed(seed)
        .set_frequency(2.8 * scatter);

    let noise_map = PlaneMapBuilder::<_, 2>::new(ridged_multi_perlin)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build();

    noise_map
}

pub fn generate_buildings_noise(seed: u32, width: usize, height: usize, scale: f64) -> NoiseMap {
    assert!(scale > 0.0, "Scale value needs to be greater than zero");
    assert!(
        width > 0 && width % (MAP_CHUNK_TILES_LENGTH as usize) == 0,
        "Width of map needs to be greater than 0 and be a multiple of the chunk side length"
    );

    let chunks_width: usize = width / (MAP_CHUNK_TILES_LENGTH as usize);
    let xy_radius: f64 = (1.0 / scale) * (chunks_width as f64);

    let worley_manhattan = Worley::default()
        .set_seed(seed)
        .set_distance_function(euclidean)
        .set_return_type(ReturnType::Value);

    let noise_map = PlaneMapBuilder::<_, 2>::new(worley_manhattan)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build();

    noise_map
}
