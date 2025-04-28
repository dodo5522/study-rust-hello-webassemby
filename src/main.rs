mod graphics;

use graphics::*;
use web_sys::wasm_bindgen::JsCast;

pub fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::debug!("Hello, world!");

  if let Some(canvas) = get_canvas_2d("canvas") {
    let points = [
      Point::new(200.0, 0.0),
      Point::new(0.0, 400.0),
      Point::new(400.0, 400.0),
    ];
    draw_triangle(&canvas, &points, false);
  }
}
