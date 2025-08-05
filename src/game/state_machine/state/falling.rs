use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct Falling;

impl RedHatBoyState<Falling> {
  pub fn frame_name(&self) -> &str {
    "Hurt"
  }

  pub fn update(mut self) -> Self {
    self.context = self.context.update(1);
    self
  }
}
