// When you load `shape_renderer` with `mod shape_renderer;`, cargo looks for the
// file "./shape_renderer.rs". If the file isn't found, it looks for a file at
// "./shape_renderer/mod.rs" (this one). Here we can load other dependencies.

use self::colors::Color;

pub mod triangle_renderer;

mod colors;
