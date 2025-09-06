use super::context::RedHatBoyContext;

mod falling;
pub use falling::Falling;
mod idle;
pub use idle::{Idle, RedHatBoyStateIdle};
mod jumping;
pub use jumping::Jumping;
pub use jumping::JumpingEndState;
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
