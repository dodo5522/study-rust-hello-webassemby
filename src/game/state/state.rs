use crate::game::engine::Point;
use crate::game::state::{
  Falling, Idle, Jumping, RedHatBoy, RedHatBoyContext, RedHatBoyState, RedHatBoyStateMachine,
  Running, Sheet, Sliding,
};
use web_sys::HtmlImageElement;

impl RedHatBoy {
  pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
    Self {
      state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::<Idle>::new()),
      sheet,
      image,
    }
  }
}

impl RedHatBoyState<Idle> {
  pub fn new() -> Self {
    Self {
      context: RedHatBoyContext {
        frame: 0,
        position: Point { x: 0, y: 475 },
        velocity: Point { x: 0, y: 0 },
      },
      _state: Idle {},
    }
  }

  pub fn run(self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context,
      _state: Running {},
    }
  }
}

impl RedHatBoyState<Running> {
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
}
