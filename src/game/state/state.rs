use crate::game::engine::Point;
use crate::game::state::{
  Falling, Idle, Jumping, KnockedOut, RedHatBoyContext, RedHatBoyState, Running, Sliding,
};

impl<S> RedHatBoyState<S> {
  pub fn context(&self) -> &RedHatBoyContext {
    &self.context
  }
}

impl RedHatBoyState<Idle> {
  pub fn new() -> Self {
    Self {
      context: RedHatBoyContext {
        frame: 0,
        position: Point { x: 0, y: 300 },
        velocity: Point { x: 0, y: 0 },
      },
      _state: Idle {},
    }
  }

  pub fn frame_name(&self) -> &str {
    "Idle"
  }

  pub fn run(self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context,
      _state: Running {},
    }
  }
}

impl RedHatBoyState<Running> {
  pub fn frame_name(&self) -> &str {
    "Run"
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
}

impl RedHatBoyState<Jumping> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }
}

impl RedHatBoyState<Sliding> {
  pub fn frame_name(&self) -> &str {
    "Slide"
  }
}

impl RedHatBoyState<Falling> {
  pub fn frame_name(&self) -> &str {
    "Hurt"
  }
}

impl RedHatBoyState<KnockedOut> {
  pub fn frame_name(&self) -> &str {
    "Dead"
  }
}
