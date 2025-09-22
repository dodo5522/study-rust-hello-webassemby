use super::{Point, Rect, Renderer};
use anyhow::Error;
use web_sys::HtmlImageElement;

pub struct Image {
  image: HtmlImageElement,
  position: Point,
}

impl Image {
  pub fn new(image: HtmlImageElement, position: Point) -> Self {
    Self { image, position }
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
}
