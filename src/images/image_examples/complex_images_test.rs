#![allow(unused_imports)]

use noise::core::worley::{distance_functions::manhattan, ReturnType};
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Add, Curve, Max, Min, Multiply};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti, Seedable, Worley};
use noise::{OpenSimplex, SuperSimplex, Terrace};

use super::MAP_CHUNK_TILES_LENGTH;

const GEN_CHUNKS_SQUARED: u32 = 6;
const XY_INDEX: (u32, u32) = (2, 0);

const CONTINENT_LACUNARITY: f64 = 2.208984375;

fn save_complex_image_stage<const DIM: usize>(id: u32, noise_fn: &impl NoiseFn<f64, DIM>) {
    const IMAGE_SIZE: (usize, usize) = (
        (GEN_CHUNKS_SQUARED as usize) * (MAP_CHUNK_TILES_LENGTH as usize),
        (GEN_CHUNKS_SQUARED as usize) * (MAP_CHUNK_TILES_LENGTH as usize),
    );
    let noise_map = PlaneMapBuilder::<_, DIM>::new(noise_fn)
        .set_size(IMAGE_SIZE.0, IMAGE_SIZE.1)
        .set_x_bounds(XY_INDEX.0 as f64, (XY_INDEX.0 + GEN_CHUNKS_SQUARED) as f64)
        .set_y_bounds(XY_INDEX.1 as f64, (XY_INDEX.1 + GEN_CHUNKS_SQUARED) as f64)
        .set_is_seamless(false)
        .build();
    println!(
        "Max: {}",
        noise_map
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
            .unwrap()
    );
    println!(
        "Min: {}",
        noise_map
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
            .unwrap()
    );
    noise_map.write_to_file(format!("example_complex_{id}.png").as_str());
}

#[test]
fn example_complex_layered() {
    let seed: u32 = 123;

    let fbm_base = Fbm::<Perlin>::new(seed + 1)
        .set_frequency(1.0)
        .set_persistence(0.5)
        .set_lacunarity(1.0 * CONTINENT_LACUNARITY)
        .set_octaves(4);
    save_complex_image_stage::<2>(1, &fbm_base);

    let fbm_curved = Curve::new(fbm_base)
        .add_control_point(-2.000, -1.000)
        .add_control_point(-1.500, -0.750)
        .add_control_point(-1.000, -0.250)
        .add_control_point(0.000, 0.000)
        .add_control_point(0.250, 0.250)
        .add_control_point(1.0000, 0.750)
        .add_control_point(2.0000, 1.000);
    save_complex_image_stage::<2>(2, &fbm_curved);
}
