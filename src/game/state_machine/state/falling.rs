use super::{KnockedOut, RedHatBoyState};

pub enum FallingEndState {
  Falling(RedHatBoyState<Falling>),
  Complete(RedHatBoyState<KnockedOut>),
}

#[derive(Copy, Clone)]
pub struct Falling;

impl Falling {
  const FRAMES: u8 = 29;
}

impl RedHatBoyState<Falling> {
  pub fn frame_name(&self) -> &str {
    "Dead"
  }

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
