use super::context::RedHatBoyContext;

mod falling;
pub use falling::*;
mod idle;
pub use idle::*;
mod jumping_in_idle;
pub use jumping_in_idle::*;
mod jumping_in_running;
pub use jumping_in_running::*;
mod knocked_out;
pub use knocked_out::*;
mod running;
pub use running::*;
mod sliding;
pub use sliding::*;

trait BaseStateStruct {
  const SPEED_JUMP: i16;
  const FRAMES: u8;
  const FRAME_NAME: &'static str;

  fn frame_name(&self) -> &'static str {
    Self::FRAME_NAME
  }
}

#[derive(Copy, Clone)]
pub struct RedHatBoyState<S: BaseStateStruct> {
  context: RedHatBoyContext,
  _state: S,
}

impl<S: BaseStateStruct> RedHatBoyState<S> {
  pub fn context(&self) -> &RedHatBoyContext {
    &self.context
  }

  pub fn frame_name(&self) -> &'static str {
    self._state.frame_name()
  }
}
