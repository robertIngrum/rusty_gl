extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use super::Color;

#[wasm_bindgen]
pub struct Renderer {
  element_id: String,
  shape: Shape,
  width: i32,
  height: i32
}
