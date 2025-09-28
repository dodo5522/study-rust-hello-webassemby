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
