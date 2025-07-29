use super::super::engine;
use super::{RedHatBoyContext, RedHatBoyState, Running};

#[derive(Copy, Clone)]
pub struct Idle;

pub type RedHatBoyStateIdle = RedHatBoyState<Idle>;

impl Idle {
  pub const FRAMES: u8 = 29;
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
