use super::{BaseStateStruct, Falling, Idle, JumpingInRunning, RedHatBoyState, Sliding};

#[derive(Copy, Clone)]
pub struct Running;

impl BaseStateStruct for Running {
  const SPEED_JUMP: i16 = -20;
  const FRAMES: u8 = 23;
  const FRAME_NAME: &'static str = "Run";
}

impl RedHatBoyState<Running> {
  pub fn update(mut self) -> Self {
    self.context = self.context.update(Running::FRAMES);
    self
  }

  pub fn jump(self) -> RedHatBoyState<JumpingInRunning> {
    RedHatBoyState {
      context: self
        .context
        .set_vertical_velocity(Running::SPEED_JUMP)
        .reset_frame(),
      _state: JumpingInRunning {},
    }
  }

  pub fn slide(self) -> RedHatBoyState<Sliding> {
    RedHatBoyState {
      context: self.context.reset_frame(),
      _state: Sliding {},
    }
  }

  pub fn stand(self) -> RedHatBoyState<Idle> {
    RedHatBoyState {
      context: self.context.stop(),
      _state: Idle {},
    }
  }

  pub fn knock_out(&self) -> RedHatBoyState<Falling> {
    RedHatBoyState {
      context: self.context.reset_frame().stop(),
      _state: Falling,
    }
  }
}
