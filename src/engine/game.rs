use web_sys::CanvasRenderingContext2d;

pub(crate) trait Game {
  fn update(&mut self);
  fn draw(&self, context: &CanvasRenderingContext2d);
}
