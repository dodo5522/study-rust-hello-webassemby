use super::{BaseStateStruct, RedHatBoyState};

#[derive(Copy, Clone)]
pub struct KnockedOut;

impl BaseStateStruct for KnockedOut {
  const SPEED_JUMP: i16 = 0;
  const FRAMES: u8 = 29;
  const FRAME_NAME: &'static str = "Dead";
}

impl RedHatBoyState<KnockedOut> {
  pub fn update(mut self) -> Self {
    self.context = self
      .context
      .update(KnockedOut::FRAMES)
      .force_frame(KnockedOut::FRAMES);
    self
  }
}
