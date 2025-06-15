pub(crate) mod browser;
pub(crate) mod loader;
pub(crate) mod renderer;

mod engine;

use crate::game::engine::renderer::Renderer;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait(?Send)]
pub(crate) trait Game {
  async fn initialize(&self) -> Result<Box<dyn Game>, Error>;
  fn update(&mut self);
  fn draw(&self, renderer: &Renderer);
}

pub(crate) struct GameLoop {
  last_frame: f64,        // 直前のフレームが リクエストされた時刻
  accumulated_delta: f32, // 最後に描画してから累積した差分時間
}
