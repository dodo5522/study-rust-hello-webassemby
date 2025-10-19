use super::{BaseStateStruct, KnockedOut, RedHatBoyState};

pub enum FallingEndState {
  Falling(RedHatBoyState<Falling>),
  Complete(RedHatBoyState<KnockedOut>),
}

#[derive(Copy, Clone)]
pub struct Falling;

impl BaseStateStruct for Falling {
  const SPEED_JUMP: i16 = 0;
  const FRAMES: u8 = 29;
  const FRAME_NAME: &'static str = "Dead";
}

impl RedHatBoyState<Falling> {
  pub fn update(mut self) -> FallingEndState {
    self.context = self.context.update(Falling::FRAMES);
    if self.context().frame() >= Falling::FRAMES {
      FallingEndState::Complete(RedHatBoyState {
        context: self.context,
        _state: KnockedOut {},
      })
    } else {
      FallingEndState::Falling(self)
    }
  }
}
