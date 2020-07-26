pub mod utils;

mod gol;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn create_universe() -> gol::Universe {
    let mut universe = gol::Universe::build_universe(20);

    universe.flip(10,5);
    universe.flip(11,5);
    universe.flip(12,5);
    universe.flip(12,4);
    universe.flip(11,3);

    universe
}
