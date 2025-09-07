use super::{Falling, Idle, JumpingInRunning, RedHatBoyState, Sliding};

#[derive(Copy, Clone)]
pub struct Running;

impl Running {
  const FRAMES: u8 = 23;
  const SPEED_JUMP: i16 = -20;
}

impl RedHatBoyState<Running> {
  pub fn frame_name(&self) -> &str {
    "Run"
  }

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

  pub fn crash(self) -> RedHatBoyState<Falling> {
    RedHatBoyState {
      context: self.context,
      _state: Falling {},
    }
  }

  pub fn stand(self) -> RedHatBoyState<Idle> {
    RedHatBoyState {
      context: self.context.stop(),
      _state: Idle {},
    }
  }
}
