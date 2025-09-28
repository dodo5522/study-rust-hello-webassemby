use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct KnockedOut;

impl RedHatBoyState<KnockedOut> {
  pub fn frame_name(&self) -> &str {
    "Dead"
  }

  pub fn update(self) -> Self {
    self
  }
}
