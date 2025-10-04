use super::engine;
use super::sheet::Sheet;
use web_sys::HtmlImageElement;

pub struct Platform {
  sheet: Sheet,
  image: HtmlImageElement,
  position: engine::Point,
}

impl Platform {
  /// Create a new platform.
  ///
  /// # Arguments
  /// * `sheet` - The sprite sheet containing the platform frame.
  /// * `image` - The HTML image element representing the platform.
  /// * `position` - The position of the platform on the screen.
  pub fn new(sheet: Sheet, image: HtmlImageElement, position: engine::Point) -> Self {
    Platform {
      sheet,
      image,
      position,
    }
  }

  /// Get Rect of the bounding box of the platform.
  ///
  /// # Returns
  /// Rect of the bounding box
  pub fn bounding_box(&self) -> engine::Rect {
    let sheet = self.sheet.frames.get("13.png").expect("Frame not found");
    engine::Rect {
      x: sheet.frame.x as f32,
      y: sheet.frame.y as f32,
      width: (sheet.frame.w * 3) as f32,
      height: sheet.frame.h as f32,
    }
  }

  /// Draw the platform on the renderer.
  ///
  /// # Arguments
  /// * `renderer` - The renderer to draw the platform on.
  pub fn draw(&self, renderer: &engine::Renderer) {
    let sheet = self.sheet.frames.get("13.png").expect("Frame not found");
    renderer
      .draw_image(
        &self.image,
        &engine::Rect {
          x: sheet.frame.x as f32,
          y: sheet.frame.y as f32,
          width: (sheet.frame.w * 3) as f32,
          height: sheet.frame.h as f32,
        },
        &engine::Rect {
          x: self.position.x as f32,
          y: self.position.y as f32,
          width: (sheet.frame.w * 3) as f32,
          height: sheet.frame.h as f32,
        },
      )
      .expect("Failed to draw image");
  }
}
