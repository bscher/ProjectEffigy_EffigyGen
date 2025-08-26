use noise::core::worley::{distance_functions::manhattan, ReturnType};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, MultiFractal, Perlin, RidgedMulti, Seedable, Worley};
use noise::{OpenSimplex, SuperSimplex, Terrace};

use super::MAP_CHUNK_TILES_LENGTH;

const LACUNARITY_CONSTANT: f64 = std::f64::consts::PI * 2.0 / 3.0;

#[test]
fn example_fbm_perlin() {
    let chunks_squared: usize = 4;
    let width: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;
    let height: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;

    let fbm = Fbm::<Perlin>::new(123)
        .set_frequency(1.0)
        .set_persistence(0.5)
        .set_lacunarity(1.0 * LACUNARITY_CONSTANT);

    let xy_radius: f64 = 1.0 * chunks_squared as f64;
    PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build()
        .write_to_file("fbm_perlin.png");
}

#[test]
fn example_open_simplex() {
    let chunks_squared: usize = 4;
    let width: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;
    let height: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;

    let open_simplex = OpenSimplex::new(123);
    let xy_radius: f64 = 1.0 * chunks_squared as f64;
    PlaneMapBuilder::<_, 2>::new(open_simplex)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build()
        .write_to_file("open_simplex.png");
}

#[test]
fn example_ridged_multi_perlin() {
    let chunks_squared: usize = 4;

    let width: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;
    let height: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;

    // pub const DEFAULT_OCTAVE_COUNT: usize = 6;
    // pub const DEFAULT_FREQUENCY: f64 = 1.0;
    // pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
    // pub const DEFAULT_PERSISTENCE: f64 = 1.0;
    // pub const DEFAULT_ATTENUATION: f64 = 2.0;
    // pub const MAX_OCTAVES: usize = 32;

    let ridged_multi_perlin = RidgedMulti::<Perlin>::default()
        .set_seed(1443)
        .set_persistence(1.0 * 1.0)
        .set_lacunarity(0.5 * LACUNARITY_CONSTANT)
        .set_attenuation(0.6 * 2.0)
        .set_frequency(1.7 * 0.25 * (chunks_squared as f64));
    let xy_radius: f64 = 0.225 * chunks_squared as f64;
    PlaneMapBuilder::<_, 2>::new(ridged_multi_perlin)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build()
        .write_to_file("ridged_multi_perlin.png");
}

#[test]
fn example_super_simplex_terrace() {
    let chunks_squared: usize = 4;
    let width: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;
    let height: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;

    let super_simplex = SuperSimplex::new(0);
    let xy_radius: f64 = 0.5 * chunks_squared as f64;
    let terrace = Terrace::<_, _, 2>::new(super_simplex)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(-0.5)
        .add_control_point(0.0)
        .add_control_point(0.0)
        .add_control_point(0.5)
        .add_control_point(0.5)
        .add_control_point(1.0)
        .invert_terraces(false);
    PlaneMapBuilder::<_, 2>::new(terrace)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build()
        .write_to_file("super_simplex_terrace.png");
}

#[test]
fn example_super_simplex() {
    let chunks_squared: usize = 4;
    let width: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;
    let height: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;

    let super_simplex = SuperSimplex::new(123);
    let xy_radius: f64 = 0.5 * chunks_squared as f64;
    PlaneMapBuilder::<_, 2>::new(super_simplex)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build()
        .write_to_file("super_simplex.png");
}

#[test]
fn example_worley_manhattan() {
    let chunks_squared: usize = 4;

    let width: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;
    let height: usize = (MAP_CHUNK_TILES_LENGTH as usize) * chunks_squared;

    let xy_radius: f64 = 2.0 * chunks_squared as f64;
    let worley_manhattan = Worley::default()
        .set_seed(7546756)
        .set_distance_function(manhattan)
        .set_return_type(ReturnType::Value);
    PlaneMapBuilder::<_, 2>::new(worley_manhattan)
        .set_size(width, height)
        .set_x_bounds(-xy_radius, xy_radius)
        .set_y_bounds(-xy_radius, xy_radius)
        .set_is_seamless(false)
        .build()
        .write_to_file("worley_manhattan.png");
}
