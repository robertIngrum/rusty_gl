mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_allock` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_allock::WeeAllock::INIT;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello world!");
}
