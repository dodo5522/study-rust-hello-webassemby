use super::{Falling, RedHatBoyState, Running};

#[derive(Copy, Clone)]
pub struct JumpingInRunning;

pub enum JumpingInRunningEndState {
  Jumping(RedHatBoyState<JumpingInRunning>),
  Complete(RedHatBoyState<Running>),
}

impl JumpingInRunning {
  const FRAMES: u8 = 35;
}

impl RedHatBoyState<JumpingInRunning> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }

  pub fn update(mut self) -> JumpingInRunningEndState {
    self.context = self.context.update(JumpingInRunning::FRAMES);

    if self.context.position().y < self.context.ground() {
      JumpingInRunningEndState::Jumping(self)
    } else {
      JumpingInRunningEndState::Complete(self.land())
    }
  }

  fn land(&mut self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context.reset_frame(),
      _state: Running {},
    }
  }

  pub fn knock_out(&self) -> RedHatBoyState<Falling> {
    RedHatBoyState {
      context: self.context.reset_frame().stop(),
      _state: Falling {},
    }
  }
}
