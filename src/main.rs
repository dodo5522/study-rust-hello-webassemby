mod browser;
mod engine;
mod game;
mod graphics;

use crate::browser::wrapper::spawn_local;
use crate::engine::game::GameLoop;
use crate::game::WalkTheDog;
use wasm_bindgen::{JsCast, JsValue};

pub fn main() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  spawn_local(async move {
    let walk_tha_dog = WalkTheDog::new();
    GameLoop::start(walk_tha_dog).await.expect("")
  });

  Ok(())
}
