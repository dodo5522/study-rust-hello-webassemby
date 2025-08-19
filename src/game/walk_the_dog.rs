use anyhow::{Error, anyhow};
use async_trait::async_trait;
use serde_wasm_bindgen::from_value;

use super::engine;
use super::red_hat_boy as rhb;
use super::sheet;

pub enum WalkTheDog {
  Loading,
  Loaded(rhb::RedHatBoy),
}

impl WalkTheDog {
  pub fn new() -> Self {
    Self::Loading
  }
}

#[async_trait(?Send)]
impl engine::Game for WalkTheDog {
  async fn initialize(&self) -> Result<Box<dyn engine::Game>, Error> {
    match self {
      WalkTheDog::Loading => {
        let image = engine::load_image("static/images/rhb.png").await?;
        let values = engine::fetch_json("static/coordinates/rhb.json").await?;
        let sheet = from_value::<sheet::Sheet>(values).map_err(|e| anyhow!(""))?;
        let rhb = rhb::RedHatBoy::new(
          sheet.clone(),
          image.clone(),
          0,
          engine::Point { x: 0, y: 0 },
          engine::Point { x: 0, y: 0 },
        );
        Ok(Box::new(WalkTheDog::Loaded(rhb)))
      }
      WalkTheDog::Loaded(_) => Err(anyhow!("")),
    }
  }

  fn update(&mut self, key_state: &engine::KeyState) {
    if let WalkTheDog::Loaded(rhb) = self {
      let mut velocity = engine::Point { x: 0, y: 0 };
      if key_state.is_pressed("ArrowUp") {
        velocity.y -= 3;
      }
      if key_state.is_pressed("ArrowDown") {
        rhb.slide();
      }
      if key_state.is_pressed("ArrowRight") {
        rhb.run_right();
      }
      if key_state.is_pressed("ArrowLeft") {
        rhb.stop();
      }
      rhb.update();
    }
  }

  fn draw(&self, renderer: &engine::Renderer) {
    if let WalkTheDog::Loaded(rhb) = self {
      rhb.draw(renderer);
    }
  }
}
