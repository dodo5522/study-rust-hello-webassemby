use super::{BaseStateStruct, Idle, RedHatBoyState};

#[derive(Copy, Clone)]
pub struct JumpingInIdle;

pub enum JumpingInIdleEndState {
  Jumping(RedHatBoyState<JumpingInIdle>),
  Complete(RedHatBoyState<Idle>),
}

impl BaseStateStruct for JumpingInIdle {
  const SPEED_JUMP: i16 = 0;
  const FRAMES: u8 = 35;
  const FRAME_NAME: &'static str = "Jump";
}

impl RedHatBoyState<JumpingInIdle> {
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
