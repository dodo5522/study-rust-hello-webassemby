use super::super::engine;
use super::{BaseStateStruct, JumpingInIdle, RedHatBoyContext, RedHatBoyState, Running};

#[derive(Copy, Clone)]
pub struct Idle;

pub type RedHatBoyStateIdle = RedHatBoyState<Idle>;

impl BaseStateStruct for Idle {
  const SPEED_JUMP: i16 = -15;
  const FRAMES: u8 = 29;
  const FRAME_NAME: &'static str = "Idle";
}

impl RedHatBoyState<Idle> {
  pub fn new(
    initial_frame: u8,
    initial_position: engine::Point,
    initial_velocity: engine::Point,
    canvas_size: engine::Size,
  ) -> Self {
    Self {
      context: RedHatBoyContext::new(
        initial_frame,
        initial_position,
        initial_velocity,
        canvas_size,
      ),
      _state: Idle {},
    }
  }

  pub fn update(mut self) -> Self {
    self.context = self.context.update(Idle::FRAMES);
    self
  }

  pub fn run(self) -> RedHatBoyState<Running> {
    RedHatBoyState {
      context: self.context.reset_frame().run_right(),
      _state: Running {},
    }
  }

  pub fn jump(self) -> RedHatBoyState<JumpingInIdle> {
    RedHatBoyState {
      context: self
        .context
        .reset_frame()
        .set_vertical_velocity(Idle::SPEED_JUMP),
      _state: JumpingInIdle {},
    }
  }
}
