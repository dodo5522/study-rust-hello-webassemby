use crate::game::engine;
use crate::game::state::{Idle, RedHatBoy, RedHatBoyState, RedHatBoyStateMachine, Sheet};
use web_sys::HtmlImageElement;

impl RedHatBoy {
  pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
    Self {
      state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::<Idle>::new()),
      sheet,
      image,
    }
  }

  pub fn draw(&self, renderer: &engine::Renderer) {
    let frame_name = format!(
      "{} ({}).png",
      self.state_machine.frame_name(),
      self.state_machine.context().frame / 3 + 1,
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
          x: self.state_machine.context().position.x.into(),
          y: self.state_machine.context().position.y.into(),
          width: cell.frame.w as f32,
          height: cell.frame.h as f32,
        },
      )
      .expect("Failed to draw image");
  }
}
