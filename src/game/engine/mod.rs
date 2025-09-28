use anyhow::Error;
use async_trait::async_trait;
use std::collections::HashMap;
use web_sys::{CanvasRenderingContext2d, KeyboardEvent};

mod browser;
pub use browser::{fetch_json, spawn_local};

mod loader;
pub use loader::load_image;

mod engine_loop;
pub use engine_loop::*;
mod image;
mod key_state;
mod rect;
mod renderer;
pub use rect::*;

pub use image::Image;

#[derive(Copy, Clone)]
pub struct Point {
  pub x: i16,
  pub y: i16,
}

#[derive(Copy, Clone)]
pub struct Size {
  pub width: u32,
  pub height: u32,
}

pub struct Renderer {
  context: CanvasRenderingContext2d,
}

enum KeyPress {
  KeyDown(KeyboardEvent),
  KeyUp(KeyboardEvent),
}

pub struct KeyState {
  pressed_keys: HashMap<String, KeyboardEvent>,
}

#[async_trait(?Send)]
pub trait Game {
  async fn initialize(&self) -> Result<Box<dyn Game>, Error>;
  fn update(&mut self, key_state: &KeyState);
  fn draw(&self, renderer: &Renderer);
}

pub struct EngineLoop {
  last_frame: f64,        // 直前のフレームが リクエストされた時刻
  accumulated_delta: f32, // 最後に描画してから累積した差分時間
}
