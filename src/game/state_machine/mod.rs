pub use self::state::RedHatBoyState;
use super::engine;

mod context;
mod state;
pub use state::*;
mod convert;
mod event;
pub use event::Event;
mod transition;

#[derive(Copy, Clone)]
pub enum RedHatBoyStateMachine {
  Falling(RedHatBoyState<Falling>),
  Idle(RedHatBoyState<Idle>),
  Running(RedHatBoyState<Running>),
  JumpingInIdle(RedHatBoyState<JumpingInIdle>),
  JumpingInRunning(RedHatBoyState<JumpingInRunning>),
  Sliding(RedHatBoyState<Sliding>),
  KnockedOut(RedHatBoyState<KnockedOut>),
}
