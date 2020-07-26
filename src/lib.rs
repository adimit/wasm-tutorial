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
    let mut universe = gol::Universe::build_universe(5);

    universe.flip(1,1);
    universe.flip(1,2);
    universe.flip(1,3);

    universe
}
