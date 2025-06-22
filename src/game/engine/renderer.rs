use anyhow::{Error, anyhow};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::engine::{Rect, Renderer};

impl Renderer {
  pub fn new(context: CanvasRenderingContext2d) -> Self {
    Self { context }
  }

  pub fn clear(&self, rect: Rect) {
    self.context.clear_rect(
      rect.x.into(),
      rect.y.into(),
      rect.width.into(),
      rect.height.into(),
    );
  }

  pub fn draw_image(
    &self,
    image: &HtmlImageElement,
    frame: &Rect,
    destination: &Rect,
  ) -> Result<(), Error> {
    self
      .context
      .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &image,
        frame.x.into(),
        frame.y.into(),
        frame.width.into(),
        frame.height.into(),
        destination.x.into(),
        destination.y.into(),
        destination.width.into(),
        destination.height.into(),
      )
      .map_err(|e| anyhow!("Error drawing {:#?} {:#?}", image, e))
  }
}
