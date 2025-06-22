use anyhow::{Error, anyhow};
use async_trait::async_trait;
use serde_wasm_bindgen::from_value;

use crate::game::WalkTheDog;
use crate::game::engine::browser::accessor::fetch_json;
use crate::game::engine::loader::load_image;
use crate::game::engine::renderer::{Rect, Renderer};
use crate::game::engine::{Game, KeyState, Point};
use crate::game::sheet::Sheet;

impl WalkTheDog {
  pub(crate) fn new() -> Self {
    Self {
      image: None,
      sheet: None,
      frame: 0,
      position: Point { x: 0, y: 0 },
    }
  }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
  async fn initialize(&self) -> Result<Box<dyn Game>, Error> {
    let image = load_image("static/images/rhb.png").await?;
    let values = fetch_json("static/coordinates/rhb.json").await?;
    let sheet = from_value::<Sheet>(values).map_err(|e| anyhow!(""))?;

    Ok(Box::new(WalkTheDog {
      image: Some(image),
      sheet: Some(sheet),
      frame: self.frame,
      position: self.position,
    }))
  }

  fn update(&mut self, key_state: &KeyState) {
    let mut velocity = Point { x: 0, y: 0 };

    if self.frame < 23 {
      self.frame += 1;
    } else {
      self.frame = 0;
    }

    if key_state.is_pressed("ArrowUp") {
      velocity.y -= 3;
    }
    if key_state.is_pressed("ArrowDown") {
      velocity.y += 3;
    }
    if key_state.is_pressed("ArrowRight") {
      velocity.x += 3;
    }
    if key_state.is_pressed("ArrowLeft") {
      velocity.x -= 3;
    }

    self.position.x += velocity.x;
    self.position.y += velocity.y;
  }

  fn draw(&self, renderer: &Renderer) {
    let current_sprite = self.frame / 3 + 1;
    let frame_name = format!("Run ({}).png", current_sprite);
    let cell = self
      .sheet
      .as_ref()
      .and_then(|sheet| sheet.frames.get(&frame_name))
      .expect(format!("{} not found", frame_name).as_str());

    renderer.clear(Rect {
      x: 0.0,
      y: 0.0,
      width: 500.0,
      height: 500.0,
    });
    self.image.as_ref().map(|image| {
      renderer
        .draw_image(
          image,
          &Rect {
            x: cell.frame.x as f32,
            y: cell.frame.y as f32,
            width: cell.frame.w as f32,
            height: cell.frame.h as f32,
          },
          &Rect {
            x: self.position.x.into(),
            y: self.position.y.into(),
            width: cell.frame.w as f32,
            height: cell.frame.h as f32,
          },
        )
        .expect("Failed to draw image");
    });
  }
}
