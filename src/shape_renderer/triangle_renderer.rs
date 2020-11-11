extern crate web_sys;

use crate::utils::logging::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
pub struct TriangleRenderer {
  width:  u32,
  height: u32,
}

#[wasm_bindgen]
impl TriangleRenderer {
  pub fn new() -> TriangleRenderer {
    let width  = 100;
    let height = 100;

    TriangleRenderer {
      width,
      height
    }
  }

  pub fn render(&self) -> Result<(), JsValue> {
    // --------------------------------
    // Get document and canvas elements
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas   = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    // --------------------------------

    // -------------------
    // Color parsing logic
    let shape_red_input   = document.get_element_by_id("shape-red").unwrap();
    let shape_green_input = document.get_element_by_id("shape-green").unwrap();
    let shape_blue_input  = document.get_element_by_id("shape-blue").unwrap();

    let shape_red_input:   web_sys::Element = shape_red_input.dyn_into::<web_sys::Element>()?;
    let shape_green_input: web_sys::Element = shape_green_input.dyn_into::<web_sys::Element>()?;
    let shape_blue_input:  web_sys::Element = shape_blue_input.dyn_into::<web_sys::Element>()?;

    // Get value of element attribute and cast to float
    let shape_red_color   = shape_red_input.get_attribute("value").unwrap().parse::<f32>().unwrap();
    let shape_green_color = shape_green_input.get_attribute("value").unwrap().parse::<f32>().unwrap();
    let shape_blue_color  = shape_blue_input.get_attribute("value").unwrap().parse::<f32>().unwrap();

    log!("Red: {}, Green: {}, Blue: {}", shape_red_color, shape_green_color, shape_blue_color);
    // -------------------

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = self.compile_shader(
      &context,
      WebGlRenderingContext::VERTEX_SHADER,
      r#"
      attribute vec4 position;
      void main() {
        gl_Position = position;
      }
      "#,
    )?;

    let frag_shader = self.compile_shader(
      &context,
      WebGlRenderingContext::FRAGMENT_SHADER,
      r#"
      void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
      }
      "#,
    )?;

    let program = self.link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("Failed to create buffer.")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
      let vert_array = js_sys::Float32Array::view(&vertices);

      context.buffer_data_with_array_buffer_view(
          WebGlRenderingContext::ARRAY_BUFFER,
          &vert_array,
          WebGlRenderingContext::STATIC_DRAW,
      );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(shape_red_color, shape_green_color, shape_blue_color, 1.0);
    // context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
      WebGlRenderingContext::TRIANGLES,
      0,
      (vertices.len() / 3) as i32,
    );

    Ok(())
  }

  fn compile_shader(
    &self,
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
  ) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object."))?;
    
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
  
    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
      Ok(shader)
    } else {
      Err(
        context
          .get_shader_info_log(&shader)
          .unwrap_or_else(|| String::from("Unknown error creating shader."))
      )
    }
  }

  fn link_program(
    &self,
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
  ) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object."))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) {
      Ok(program)
    } else {
      Err(
        context
          .get_program_info_log(&program)
          .unwrap_or_else(|| String::from("Unknown error creating program object."))
      )
    }
  }
}
