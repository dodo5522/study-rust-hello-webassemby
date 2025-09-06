use super::context::RedHatBoyContext;

mod falling;
pub use falling::Falling;
mod idle;
pub use idle::{Idle, RedHatBoyStateIdle};
mod jumping_in_idle;
pub use jumping_in_idle::{JumpingInIdle, JumpingInIdleEndState};
mod jumping_in_running;
pub use jumping_in_running::{JumpingInRunning, JumpingInRunningEndState};
mod knocked_out;
pub use knocked_out::KnockedOut;
mod running;
pub use running::Running;
mod sliding;
pub use sliding::Sliding;
pub use sliding::SlidingEndState;

#[derive(Copy, Clone)]
pub struct RedHatBoyState<S> {
  context: RedHatBoyContext,
  _state: S,
}

impl<S> RedHatBoyState<S> {
  pub fn context(&self) -> &RedHatBoyContext {
    &self.context
  }
}
