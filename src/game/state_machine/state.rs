use super::RedHatBoyContext;
use super::engine;
use super::{Falling, Idle, Jumping, KnockedOut, Running, Sliding};

pub type RedHatBoyStateIdle = RedHatBoyState<Idle>;

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

impl Idle {
  pub const FRAMES: u8 = 29;
}

impl Running {
  pub const FRAMES: u8 = 23;
}

impl Sliding {
  pub const FRAMES: u8 = 14;
}

impl RedHatBoyState<Idle> {
  pub fn new(
    initial_frame: u8,
    initial_position: engine::Point,
    initial_velocity: engine::Point,
  ) -> Self {
    Self {
      context: RedHatBoyContext::new(initial_frame, initial_position, initial_velocity),
      _state: Idle {},
    }
  }

  pub fn frame_name(&self) -> &str {
    "Idle"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(Idle::FRAMES);
  }

  pub fn run(self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context.reset_frame().run_right(),
      _state: Running {},
    }
  }
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
}

impl RedHatBoyState<Jumping> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(1);
  }
}

impl RedHatBoyState<Sliding> {
  pub fn frame_name(&self) -> &str {
    "Slide"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(Sliding::FRAMES);
  }
}

impl RedHatBoyState<Falling> {
  pub fn frame_name(&self) -> &str {
    "Hurt"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(1);
  }
}

impl RedHatBoyState<KnockedOut> {
  pub fn frame_name(&self) -> &str {
    "Dead"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(1);
  }
}
