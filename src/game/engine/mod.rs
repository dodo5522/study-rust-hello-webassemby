pub(crate) mod browser;
pub(crate) mod loader;
pub(crate) mod renderer;

mod engine;

use crate::game::engine::renderer::Renderer;
use anyhow::Error;
use async_trait::async_trait;
use std::collections::HashMap;
use web_sys::KeyboardEvent;

enum KeyPress {
  KeyDown(KeyboardEvent),
  KeyUp(KeyboardEvent),
}

pub(crate) struct KeyState {
  pressed_keys: HashMap<String, KeyboardEvent>,
}

#[async_trait(?Send)]
pub(crate) trait Game {
  async fn initialize(&self) -> Result<Box<dyn Game>, Error>;
  fn update(&mut self, key_state: &KeyState);
  fn draw(&self, renderer: &Renderer);
}

pub(crate) struct GameLoop {
  last_frame: f64,        // 直前のフレームが リクエストされた時刻
  accumulated_delta: f32, // 最後に描画してから累積した差分時間
}

#[derive(Copy, Clone)]
pub(crate) struct Point {
  pub x: i16,
  pub y: i16,
}
