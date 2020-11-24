extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use super::{Color, RegularPolygon, Vertex};

#[wasm_bindgen]
pub struct TriangleRenderer {
  element_id: String,
  width: i32,
  height: i32
}

#[wasm_bindgen]
impl TriangleRenderer {
  pub fn new(element_id: String, width: i32, height: i32) -> TriangleRenderer {
    TriangleRenderer { element_id, width, height }
  }

  pub fn setViewport(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas   = document.get_element_by_id(&self.element_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
      .get_context("webgl")?
      .unwrap()
      .dyn_into::<WebGlRenderingContext>()?;

    context.viewport(x, y, width, height);

    Ok(())
  }

  pub fn render(&self) -> Result<(), JsValue> {
    // Get document and canvas elements
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas   = document.get_element_by_id(&self.element_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let backgroundColor = self.fetchBackgroundColor(&document).unwrap();
    let shapeColor      = self.fetchShapeColor(&document).unwrap();
    let vertex_count    = self.fetchVertexCount(&document).unwrap();

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    context.viewport(0, 0, self.width, self.height);

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
      1.0 - shapeColor.red,
      1.0 - shapeColor.green,
      1.0 - shapeColor.blue,
    );
    let frag_shader = self.compile_shader(
      &context,
      WebGlRenderingContext::FRAGMENT_SHADER,
      &frag_shader_source
    )?;

    let program = self.link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    let triangle = RegularPolygon {
      center: Vertex { x: 0.0, y: 0.0 },
      radius: 0.7,
      vertex_count: vertex_count
    };
    
    let mut vertices = Vec::new();
    for n in 0..(triangle.vertex_count) {
      vertices.push(triangle.coordinate(n));
    }
    let mut vertex_vector = Vec::new();
    for n in 0..(triangle.vertex_count) {
      vertex_vector.push(vertices[n as usize].x);
      vertex_vector.push(vertices[n as usize].y);
      vertex_vector.push(0.0);
    }
    let vertex_array: &[_] = &vertex_vector;

    // Note: Use this to debug coordinates
    // log!("{:?}", vertex_array);

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
      let vert_array = js_sys::Float32Array::view(&vertex_array);

      context.buffer_data_with_array_buffer_view(
          WebGlRenderingContext::ARRAY_BUFFER,
          &vert_array,
          WebGlRenderingContext::STATIC_DRAW,
      );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(
      1.0 - backgroundColor.red,
      1.0 - backgroundColor.green,
      1.0 - backgroundColor.blue,
      1.0
    );
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
      WebGlRenderingContext::TRIANGLE_FAN,
      0,
      (vertex_array.len() / 3) as i32,
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

  fn fetchVertexCount(&self, document: &web_sys::Document) -> Result<u32, JsValue> {
    let input = document.get_element_by_id("vertex-count").unwrap();
    let input: web_sys::HtmlInputElement = input.dyn_into::<web_sys::HtmlInputElement>()?;

    Ok(input.value().parse::<u32>().unwrap())
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
