pub mod utils;

mod gol;

use wasm_bindgen::prelude::*;
use rand::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn create_universe() -> gol::Universe {
    let mut universe = gol::Universe::build_universe(40);
    let mut rng = rand::thread_rng();

    for x in 0..(universe.width()) {
        for y in 0..(universe.height()) {
            if rng.gen() {universe.flip(x,y); }
        }
    }

    universe
}
