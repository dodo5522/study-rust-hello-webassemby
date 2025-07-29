use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct Falling;

impl RedHatBoyState<Falling> {
  pub fn frame_name(&self) -> &str {
    "Hurt"
  }

  pub fn update(&mut self) {
    self.context = self.context.update(1);
  }
}
