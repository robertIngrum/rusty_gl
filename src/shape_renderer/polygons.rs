use std::f32::consts::PI;

pub struct Vertex {
  pub x: f32,
  pub y: f32,
}

pub struct RegularPolygon {
  pub center: Vertex,
  pub radius: f32,
  pub vertex_count: u32,
}

impl RegularPolygon {
  pub fn coordinate(&self, index: u32) -> Vertex {
    let radians = 2.0 * PI * (index as f32) / self.vertex_count as f32;
    let x = self.center.x + self.radius * radians.sin();
    let y = self.center.y + self.radius * radians.cos();

    Vertex { x, y }
  }
}
