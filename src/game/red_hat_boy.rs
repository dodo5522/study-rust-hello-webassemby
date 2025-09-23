use super::engine;
use super::sheet;
use super::state_machine as state_m;
use web_sys::HtmlImageElement;

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
    let cell = self.current_sprite().expect("Frame not found");
    let bounding_box = self.bounding_box();

    renderer
      .draw_image(
        &self.image,
        &engine::Rect {
          x: cell.frame.x as f32,
          y: cell.frame.y as f32,
          width: cell.frame.w as f32,
          height: cell.frame.h as f32,
        },
        &bounding_box,
      )
      .expect("Failed to draw image");

    #[cfg(feature = "bounding-boxes")]
    renderer
      .draw_rect(&bounding_box)
      .expect("Cannot render bounding box")
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

  fn frame_name(&self) -> String {
    format!(
      "{} ({}).png",
      self.state_machine.frame_name(),
      self.state_machine.context().frame() / 3 + 1,
    )
  }

  fn current_sprite(&self) -> Option<&sheet::SheetCell> {
    self.sheet.frames.get(&self.frame_name())
  }

  pub fn bounding_box(&self) -> engine::Rect {
    let cell = self.current_sprite().expect("Frame not found");
    engine::Rect {
      x: (self.state_machine.context().position().x + cell.sprite_source_size.x as i16).into(),
      y: (self.state_machine.context().position().y + cell.sprite_source_size.y as i16).into(),
      width: cell.frame.w as f32,
      height: cell.frame.h as f32,
    }
  }
}
