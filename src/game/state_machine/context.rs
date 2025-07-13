use super::engine;

#[derive(Copy, Clone)]
pub struct RedHatBoyContext {
  frame: u8,
  position: engine::Point,
  velocity: engine::Point,
}

impl RedHatBoyContext {
  pub fn new(frame: u8, position: engine::Point, velocity: engine::Point) -> Self {
    RedHatBoyContext {
      frame,
      position,
      velocity,
    }
  }

  pub fn frame(&self) -> u8 {
    self.frame
  }

  pub fn position(&self) -> engine::Point {
    self.position
  }
}
