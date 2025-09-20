use super::{Point, Renderer};
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
    renderer.draw_entire_image(&self.image, &self.position)
  }
}
