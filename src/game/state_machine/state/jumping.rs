use crate::game::state_machine::Event::Jump;
use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct Jumping;

impl Jumping {
  const FRAMES: u8 = 35;
}

impl RedHatBoyState<Jumping> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }

  pub fn update(mut self) -> Self {
    self.context = self.context.update(Jumping::FRAMES);
    self
  }
}
