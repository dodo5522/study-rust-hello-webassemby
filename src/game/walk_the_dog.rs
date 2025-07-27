use anyhow::{Error, anyhow};
use async_trait::async_trait;
use serde_wasm_bindgen::from_value;
use web_sys::HtmlImageElement;

use super::engine;
use super::red_hat_boy as rhb;
use super::sheet;

pub struct WalkTheDog {
  image: Option<HtmlImageElement>,
  sheet: Option<sheet::Sheet>,
  frame: i32,
  position: engine::Point,
  rhb: Option<rhb::RedHatBoy>,
}

impl WalkTheDog {
  pub fn new() -> Self {
    Self {
      image: None,
      sheet: None,
      frame: 0,
      position: engine::Point { x: 0, y: 0 },
      rhb: None,
    }
  }
}

#[async_trait(?Send)]
impl engine::Game for WalkTheDog {
  async fn initialize(&self) -> Result<Box<dyn engine::Game>, Error> {
    let image = engine::load_image("static/images/rhb.png").await?;
    let values = engine::fetch_json("static/coordinates/rhb.json").await?;
    let sheet = from_value::<sheet::Sheet>(values).map_err(|e| anyhow!(""))?;

    Ok(Box::new(WalkTheDog {
      image: Some(image.clone()),
      sheet: Some(sheet.clone()),
      frame: self.frame,
      position: self.position,
      rhb: Some(rhb::RedHatBoy::new(
        sheet.clone(),
        image.clone(),
        0,
        self.position,
        engine::Point { x: 0, y: 0 },
      )),
    }))
  }

  fn update(&mut self, key_state: &engine::KeyState) {
    let mut velocity = engine::Point { x: 0, y: 0 };
    if key_state.is_pressed("ArrowUp") {
      velocity.y -= 3;
    }
    if key_state.is_pressed("ArrowDown") {
      velocity.y += 3;
    }
    if key_state.is_pressed("ArrowRight") {
      self
        .rhb
        .as_mut()
        .expect("Cannot get RedHatBoy instance")
        .run_right();
    }
    if key_state.is_pressed("ArrowLeft") {
      velocity.x -= 3;
    }

    self.rhb.as_mut().expect("Cannot get mutable rhb").update();
  }

  fn draw(&self, renderer: &engine::Renderer) {
    self.rhb.as_ref().map(|ref_rhb| ref_rhb.draw(renderer));
  }
}
