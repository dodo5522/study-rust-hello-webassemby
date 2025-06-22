use wasm_bindgen::JsValue;

mod game;

pub fn main() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  game::spawn_local(async move {
    let walk_tha_dog = game::WalkTheDog::new();
    game::GameLoop::start(walk_tha_dog).await.expect("")
  });

  Ok(())
}
