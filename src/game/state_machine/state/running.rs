use super::{Falling, Idle, Jumping, RedHatBoyState, Sliding};

#[derive(Copy, Clone)]
pub struct Running;

impl Running {
  pub const FRAMES: u8 = 23;
}

impl RedHatBoyState<Running> {
  pub fn frame_name(&self) -> &str {
    "Run"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(Running::FRAMES);
  }

  pub fn jump(self) -> RedHatBoyState<Jumping> {
    RedHatBoyState {
      context: self.context,
      _state: Jumping {},
    }
  }

  pub fn slide(self) -> RedHatBoyState<Sliding> {
    RedHatBoyState {
      context: self.context,
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
