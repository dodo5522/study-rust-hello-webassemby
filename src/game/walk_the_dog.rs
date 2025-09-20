use anyhow::{Error, anyhow};
use async_trait::async_trait;
use serde_wasm_bindgen::from_value;

use super::engine;
use super::red_hat_boy as rhb;
use super::sheet;
use super::walk;

pub enum WalkTheDog {
  Loading,
  Loaded(walk::Walk),
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
        let player = engine::load_image("static/images/rhb.png").await?;
        let background = engine::load_image("static/images/BG.png").await?;
        let values = engine::fetch_json("static/coordinates/rhb.json").await?;
        let sheet = from_value::<sheet::Sheet>(values).map_err(|e| anyhow!(""))?;
        let rhb = rhb::RedHatBoy::new(
          sheet.clone(),
          player.clone(),
          0,
          engine::Point { x: 0, y: 350 },
          engine::Point { x: 0, y: 0 },
        );
        Ok(Box::new(WalkTheDog::Loaded(walk::Walk {
          boy: rhb,
          background: engine::Image::new(background, engine::Point { x: 0, y: 0 }),
        })))
      }
      WalkTheDog::Loaded(_) => Err(anyhow!("")),
    }
  }

  fn update(&mut self, key_state: &engine::KeyState) {
    if let WalkTheDog::Loaded(walk) = self {
      let mut velocity = engine::Point { x: 0, y: 0 };
      if key_state.is_pressed("ArrowUp") {
        velocity.y -= 3;
      }
      if key_state.is_pressed("ArrowDown") {
        walk.boy.slide();
      }
      if key_state.is_pressed("ArrowRight") {
        walk.boy.run_right();
      }
      if key_state.is_pressed("ArrowLeft") {
        walk.boy.stop();
      }
      if key_state.is_pressed("Space") {
        walk.boy.jump();
      }
      walk.boy.update();
    }
  }

  fn draw(&self, renderer: &engine::Renderer) {
    if let WalkTheDog::Loaded(walk) = self {
      let _ = walk.background.draw(renderer);
      walk.boy.draw(renderer);
    }
  }
}
