mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate opencv;
extern crate enigo;

#[wasm_bindgen]
// mod mouse;
mod timm_barth;
mod eye_tracker;

#[wasm_bindgen]
fn main() {
    eye_tracker::run().expect("Runtime error")
}
