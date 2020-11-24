// --------------------- //
// INTERNAL DEPENDENCIES //
// --------------------- //
use wasm_bindgen::prelude::*;

#[macro_use]
mod utils {
  #[macro_use]
  pub mod logging;
  pub mod config;
}

// --------------------- //
//    PUBLIC  MODULES    //
// --------------------- //
pub mod shape_renderer;

// --------------------- //
//     STARTUP LOGIC     //
// --------------------- //
#[wasm_bindgen(start)]
pub fn start() {
  // Enable console debugging
  utils::config::set_panic_hook();
}
