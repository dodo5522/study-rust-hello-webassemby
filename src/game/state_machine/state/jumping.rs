use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct Jumping;

impl RedHatBoyState<Jumping> {
  pub fn frame_name(&self) -> &str {
    "Jump"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(1);
  }
}
