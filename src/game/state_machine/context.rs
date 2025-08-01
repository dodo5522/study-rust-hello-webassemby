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

  pub fn update(mut self, frame_count: u8) -> Self {
    if self.frame < frame_count {
      self.frame += 1;
    } else {
      self.frame = 0;
    }
    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;
    self
  }

  pub fn reset_frame(mut self) -> Self {
    self.frame = 0;
    self
  }

  pub fn run_right(mut self) -> Self {
    self.velocity.x += 3;
    self
  }
}
