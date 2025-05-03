mod graphics;

use graphics::canvas::get_canvas_2d;
use graphics::color::Color;
use graphics::point::Point;
use graphics::triangle::draw_sierpinski;

pub fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::debug!("Hello, world!");

  if let Some(canvas) = get_canvas_2d("canvas") {
    let points = [
      Point::new(250.0, 0.0),
      Point::new(0.0, 500.0),
      Point::new(500.0, 500.0),
    ];
    let color = Color::random();
    draw_sierpinski(&canvas, &points, &color, 6, true);
  }
}
