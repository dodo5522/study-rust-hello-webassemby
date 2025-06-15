pub mod sheet;

use anyhow::{Error, anyhow};
use async_trait::async_trait;
use serde_wasm_bindgen::from_value;
use web_sys::HtmlImageElement;

use crate::browser::accessor::fetch_json;
use crate::engine::game::Game;
use crate::engine::loader::load_image;
use crate::graphics::rect::Rect;
use sheet::Sheet;

pub struct WalkTheDog {
  image: Option<HtmlImageElement>,
  sheet: Option<Sheet>,
  frame: i32,
}

impl WalkTheDog {
  pub(crate) fn new() -> Self {
    Self {
      image: None,
      sheet: None,
      frame: 0,
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
      frame: 0,
    }))
  }

  fn update(&mut self) {
    if self.frame < 23 {
      self.frame += 1;
    } else {
      self.frame = 0;
    }
  }

  fn draw(&self, renderer: &crate::engine::renderer::Renderer) {
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
            x: 250.0,
            y: 250.0,
            width: cell.frame.w as f32,
            height: cell.frame.h as f32,
          },
        )
        .expect("Failed to draw image");
    });
  }
}
