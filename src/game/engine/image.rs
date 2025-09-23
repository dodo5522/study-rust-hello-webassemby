use super::{Point, Rect, Renderer};
use anyhow::Error;
use web_sys::HtmlImageElement;

pub struct Image {
  image: HtmlImageElement,
  position: Point,
  bounding_box: Rect,
}

impl Image {
  pub fn new(image: HtmlImageElement, position: Point) -> Self {
    let bounding_box = Rect {
      x: position.x.into(),
      y: position.y.into(),
      width: image.width() as f32,
      height: image.height() as f32,
    };
    Self {
      image,
      position,
      bounding_box,
    }
  }

  pub fn draw(&self, renderer: &Renderer) -> Result<(), Error> {
    let _ = renderer.draw_entire_image(&self.image, &self.position);

    #[cfg(feature = "bounding-boxes")]
    let _ = renderer.draw_rect(&Rect {
      x: self.position.x as f32,
      y: self.position.y as f32,
      width: self.image.width() as f32,
      height: self.image.height() as f32,
    });

    Ok(())
  }

  pub fn bounding_box(&self) -> Rect {
    self.bounding_box
  }
}
