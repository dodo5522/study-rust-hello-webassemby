use super::{Idle, RedHatBoyState};

#[derive(Copy, Clone)]
pub struct Sliding;

impl Sliding {
  pub const FRAMES: u8 = 14;
}

impl RedHatBoyState<Sliding> {
  pub fn frame_name(&self) -> &str {
    "Slide"
  }

  pub fn update(mut self) -> Self {
    self.context = self.context.update(Sliding::FRAMES);
    self
  }

  pub fn stand(&self) -> RedHatBoyState<Idle> {
    RedHatBoyState {
      context: self.context.stop(),
      _state: Idle {},
    }
  }
}
