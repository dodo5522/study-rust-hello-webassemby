use super::RedHatBoyState;
use super::Running;

#[derive(Copy, Clone)]
pub struct Jumping;

pub enum JumpingEndState {
  Jumping(RedHatBoyState<Jumping>),
  Complete(RedHatBoyState<Running>),
}

impl Jumping {
  const FRAMES: u8 = 35;
}

impl RedHatBoyState<Jumping> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }

  pub fn update(mut self) -> JumpingEndState {
    self.context = self.context.update(Jumping::FRAMES);

    if self.context.position().y < self.context.ground() {
      JumpingEndState::Jumping(self)
    } else {
      JumpingEndState::Complete(self.land())
    }
  }

  fn land(&mut self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context.reset_frame(),
      _state: Running {},
    }
  }
}
