use super::Idle;
use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct JumpingInIdle;

pub enum JumpingInIdleEndState {
  Jumping(RedHatBoyState<JumpingInIdle>),
  Complete(RedHatBoyState<Idle>),
}

impl JumpingInIdle {
  const FRAMES: u8 = 35;
}

impl RedHatBoyState<JumpingInIdle> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }

  pub fn update(mut self) -> JumpingInIdleEndState {
    self.context = self.context.update(JumpingInIdle::FRAMES);

    if self.context.position().y < self.context.ground() {
      JumpingInIdleEndState::Jumping(self)
    } else {
      JumpingInIdleEndState::Complete(self.land())
    }
  }

  fn land(&mut self) -> RedHatBoyState<Idle> {
    RedHatBoyState {
      context: self.context.reset_frame(),
      _state: Idle {},
    }
  }
}
