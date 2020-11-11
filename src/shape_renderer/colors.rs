pub struct Color {
  pub red:   f32,
  pub green: f32,
  pub blue:  f32,
}

impl Color {
  pub fn new(red: f32, green: f32, blue: f32) -> Color {
    Color { red, green, blue }
  }
}
