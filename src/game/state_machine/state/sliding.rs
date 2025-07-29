use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct Sliding;

impl Sliding {
  pub const FRAMES: u8 = 14;
}

impl RedHatBoyState<Sliding> {
  pub fn frame_name(&self) -> &str {
    "Slide"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(Sliding::FRAMES);
  }
}
