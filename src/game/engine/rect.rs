#[derive(Copy, Clone)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Rect {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect {
      x,
      y,
      width,
      height,
    }
  }

  /// Detect collision with another Rect object.
  ///
  /// # Arguments
  /// * `other` - Another Rect object
  /// # Returns
  /// Collision detected if true
  pub fn intersect(&self, other: Rect) -> bool {
    self.x < (other.x + other.width)
      && (self.x + self.width) > other.x
      && self.y < (other.y + other.height)
      && self.y + self.height > other.y
  }
}
