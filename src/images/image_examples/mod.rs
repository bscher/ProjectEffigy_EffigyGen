pub use effigy_shared::game::map::MAP_CHUNK_TILES_LENGTH;

#[cfg(all(test, feature = "examples-complex"))]
mod complex_images_test;
#[cfg(all(test, feature = "examples-simple"))]
mod simple_images_tests;
