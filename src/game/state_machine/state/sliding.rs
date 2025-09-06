use super::{RedHatBoyState, Running};

#[derive(Copy, Clone)]
pub struct Sliding;

pub enum SlidingEndState {
  Sliding(RedHatBoyState<Sliding>),
  Complete(RedHatBoyState<Running>),
}

impl Sliding {
  pub const FRAMES: u8 = 14;
}

impl RedHatBoyState<Sliding> {
  pub fn frame_name(&self) -> &str {
    "Slide"
  }

  pub fn update(mut self) -> SlidingEndState {
    self.context = self.context.update(Sliding::FRAMES);

    if self.context.frame() >= Sliding::FRAMES {
      SlidingEndState::Complete(self.stand())
    } else {
      SlidingEndState::Sliding(self)
    }
  }

  fn stand(&self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context.reset_frame(),
      _state: Running {},
    }
  }
}
