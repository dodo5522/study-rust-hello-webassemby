use super::engine;
use super::platform::Platform;
use super::red_hat_boy::RedHatBoy;

pub struct Walk {
  pub boy: RedHatBoy,
  pub background: engine::Image,
  pub stone: engine::Image,
  pub platform: Platform,
}
