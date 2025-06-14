use anyhow::Error;
use async_trait::async_trait;
use web_sys::CanvasRenderingContext2d;

#[async_trait]
pub(crate) trait Game {
  async fn initialize(&self) -> Result<Box<dyn Game>, Error>;
  fn update(&mut self);
  fn draw(&self, context: &CanvasRenderingContext2d);
}
