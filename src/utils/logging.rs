use wasm_bindgen::prelude::*;

// JS external method utils and bindings
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  pub fn console_log(s: &str);

  #[wasm_bindgen]
  pub fn alert(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
//   usage: `log!(str, interpolated_args*)` 
#[macro_export]
macro_rules! log {
  ( $( $t:tt )* ) => {
    // Because of the way macros work, we need to always explicitly require this method.
    use crate::utils::logging::console_log;

    console_log(&format_args!( $( $t )* ).to_string());
  }  
}  
