use super::{BaseStateStruct, Falling, RedHatBoyState, Running};

#[derive(Copy, Clone)]
pub struct Sliding;

pub enum SlidingEndState {
  Sliding(RedHatBoyState<Sliding>),
  Complete(RedHatBoyState<Running>),
}

impl BaseStateStruct for Sliding {
  const SPEED_JUMP: i16 = 0;
  const FRAMES: u8 = 14;
  const FRAME_NAME: &'static str = "Slide";
}

impl RedHatBoyState<Sliding> {
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

  pub fn knock_out(&self) -> RedHatBoyState<Falling> {
    RedHatBoyState {
      context: self.context.reset_frame().stop(),
      _state: Falling,
    }
  }
}
