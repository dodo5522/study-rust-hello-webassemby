use super::engine;

#[derive(Copy, Clone)]
pub struct RedHatBoyContext {
  ground: i16,
  frame: u8,
  position: engine::Point,
  velocity: engine::Point,
}

impl RedHatBoyContext {
  const GRAVITY: i16 = 1;

  pub fn new(
    frame: u8,
    position: engine::Point,
    velocity: engine::Point,
    canvas_size: engine::Size,
  ) -> Self {
    RedHatBoyContext {
      ground: canvas_size.height as i16 - 120,
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

  pub fn ground(&self) -> i16 {
    self.ground
  }

  pub fn update(mut self, frame_count: u8) -> Self {
    if self.frame < frame_count {
      self.frame += 1;
    } else {
      self.frame = 0;
    }
    self.velocity.y += Self::GRAVITY;

    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;

    if self.position.y > self.ground {
      self.position.y = self.ground;
    }
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

  pub fn stop(mut self) -> Self {
    self.velocity.x = 0;
    self
  }

  pub fn set_vertical_velocity(mut self, y: i16) -> Self {
    self.velocity.y = y;
    self
  }
}
