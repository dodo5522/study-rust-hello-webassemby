use anyhow::Error;
use async_trait::async_trait;
use std::collections::HashMap;
use web_sys::{CanvasRenderingContext2d, KeyboardEvent};

mod browser;
pub use browser::{fetch_json, spawn_local};

mod loader;
pub use loader::load_image;

mod engine_loop;
mod key_state;
mod renderer;

#[derive(Copy, Clone)]
pub(crate) struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

#[derive(Copy, Clone)]
pub(crate) struct Point {
  pub x: i16,
  pub y: i16,
}

pub(crate) struct Renderer {
  context: CanvasRenderingContext2d,
}

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

pub(crate) struct EngineLoop {
  last_frame: f64,        // 直前のフレームが リクエストされた時刻
  accumulated_delta: f32, // 最後に描画してから累積した差分時間
}
