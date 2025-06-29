use web_sys::HtmlImageElement;

mod sheet;
mod state;
mod state_machine;
use crate::game::engine::Point;
pub use sheet::Sheet;

#[derive(Copy, Clone)]
struct Idle;
#[derive(Copy, Clone)]
struct Running;
#[derive(Copy, Clone)]
struct Jumping;
#[derive(Copy, Clone)]
struct Sliding;
#[derive(Copy, Clone)]
struct Falling;
#[derive(Copy, Clone)]
struct KnockedOut;

enum RedHatBoyStateMachine {
  Idle(RedHatBoyState<Idle>),
  Running(RedHatBoyState<Running>),
  Jumping(RedHatBoyState<Jumping>),
  Sliding(RedHatBoyState<Sliding>),
  Falling(RedHatBoyState<Falling>),
  KnockedOut(RedHatBoyState<KnockedOut>),
}

#[derive(Copy, Clone)]
struct RedHatBoyContext {
  frame: u8,
  position: Point,
  velocity: Point,
}

#[derive(Copy, Clone)]
pub struct RedHatBoyState<S> {
  context: RedHatBoyContext,
  _state: S,
}

pub struct RedHatBoy {
  state_machine: RedHatBoyStateMachine,
  sheet: Sheet,
  image: HtmlImageElement,
}
