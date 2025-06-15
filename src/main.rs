use wasm_bindgen::{JsCast, JsValue};

mod game;
use game::WalkTheDog;
use game::engine::GameLoop;
use game::engine::browser::wrapper::spawn_local;

pub fn main() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  spawn_local(async move {
    let walk_tha_dog = WalkTheDog::new();
    GameLoop::start(walk_tha_dog).await.expect("")
  });

  Ok(())
}
