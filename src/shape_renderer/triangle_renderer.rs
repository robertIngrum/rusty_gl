extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use super::Color;

#[wasm_bindgen]
pub struct TriangleRenderer {
  width: u32,
  height: u32
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
    // Get document and canvas elements
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas   = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let backgroundColor = self.fetchBackgroundColor(&document).unwrap();
    let shapeColor      = self.fetchShapeColor(&document).unwrap();

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

    let frag_shader_source = format!(
      r#"
      void main() {{
        gl_FragColor = vec4({}, {}, {}, 1.0);
      }}
      "#,
      shapeColor.red,
      shapeColor.green,
      shapeColor.blue,
    );
    let frag_shader = self.compile_shader(
      &context,
      WebGlRenderingContext::FRAGMENT_SHADER,
      &frag_shader_source
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

    context.clear_color(
      backgroundColor.red,
      backgroundColor.green,
      backgroundColor.blue,
      1.0
    );
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

  fn fetchShapeColor(&self, document: &web_sys::Document) -> Result<Color, JsValue> {
    let red   = self.parseInputColorValue(document, "shape-red").unwrap();
    let green = self.parseInputColorValue(document, "shape-green").unwrap();
    let blue  = self.parseInputColorValue(document, "shape-blue").unwrap();

    Ok(Color::new(red, green, blue))
  }

  fn fetchBackgroundColor(&self, document: &web_sys::Document) -> Result<Color, JsValue> {
    let red   = self.parseInputColorValue(document, "background-red").unwrap();
    let green = self.parseInputColorValue(document, "background-green").unwrap();
    let blue  = self.parseInputColorValue(document, "background-blue").unwrap();

    Ok(Color::new(red, green, blue))
  }

  fn parseInputColorValue(&self, document: &web_sys::Document, id: &str) -> Result<f32, JsValue> {
    let input = document.get_element_by_id(id).unwrap();
    let input: web_sys::HtmlInputElement = input.dyn_into::<web_sys::HtmlInputElement>()?;

    Ok(input.value().parse::<f32>().unwrap())
  }
}
