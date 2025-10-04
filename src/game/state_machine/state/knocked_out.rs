use super::RedHatBoyState;

#[derive(Copy, Clone)]
pub struct KnockedOut;

impl KnockedOut {
  const FRAMES: u8 = 29;
}

impl RedHatBoyState<KnockedOut> {
  pub fn frame_name(&self) -> &str {
    "Dead"
  }

  pub fn update(mut self) -> Self {
    self.context = self
      .context
      .update(KnockedOut::FRAMES)
      .force_frame(KnockedOut::FRAMES);
    self
  }
}
