use web_sys::HtmlImageElement;

use super::engine;
use super::sheet;
use super::state_machine as state_m;

pub struct RedHatBoy {
  state_machine: state_m::RedHatBoyStateMachine,
  sheet: sheet::Sheet,
  image: HtmlImageElement,
}

impl RedHatBoy {
  pub fn new(
    sheet: sheet::Sheet,
    image: HtmlImageElement,
    initial_frame: u8,
    initial_position: engine::Point,
    initial_velocity: engine::Point,
    canvas_size: engine::Size,
  ) -> Self {
    Self {
      state_machine: state_m::RedHatBoyStateMachine::Idle(state_m::RedHatBoyStateIdle::new(
        initial_frame,
        initial_position,
        initial_velocity,
        canvas_size,
      )),
      sheet,
      image,
    }
  }

  pub fn draw(&self, renderer: &engine::Renderer) {
    let frame_name = format!(
      "{} ({}).png",
      self.state_machine.frame_name(),
      self.state_machine.context().frame() / 3 + 1,
    );
    let cell = self
      .sheet
      .frames
      .get(&frame_name)
      .expect(format!("{} not found", frame_name).as_str());

    renderer
      .draw_image(
        &self.image,
        &engine::Rect {
          x: cell.frame.x as f32,
          y: cell.frame.y as f32,
          width: cell.frame.w as f32,
          height: cell.frame.h as f32,
        },
        &engine::Rect {
          x: self.state_machine.context().position().x.into(),
          y: self.state_machine.context().position().y.into(),
          width: cell.frame.w as f32,
          height: cell.frame.h as f32,
        },
      )
      .expect("Failed to draw image");
  }

  pub fn update(&mut self) {
    self.state_machine = self.state_machine.update();
  }

  pub fn run_right(&mut self) {
    self.state_machine = self.state_machine.transition(state_m::Event::Run);
  }

  pub fn slide(&mut self) {
    self.state_machine = self.state_machine.transition(state_m::Event::Slide);
  }

  pub fn stop(&mut self) {
    self.state_machine = self.state_machine.transition(state_m::Event::Stop);
  }

  pub fn jump(&mut self) {
    self.state_machine = self.state_machine.transition(state_m::Event::Jump);
  }
}
