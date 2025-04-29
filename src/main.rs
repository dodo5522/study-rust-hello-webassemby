mod graphics;

use graphics::*;
use web_sys::wasm_bindgen::JsCast;

pub fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::debug!("Hello, world!");

  if let Some(canvas) = get_canvas_2d("canvas") {
    let points = [
      Point::new(250.0, 0.0),
      Point::new(0.0, 500.0),
      Point::new(500.0, 500.0),
    ];
    draw_sierpinski(&canvas, &points, 3, false);
  }
}
