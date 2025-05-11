use rand::{Rng, rng};

pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl Color {
  pub fn new(red: u8, green: u8, blue: u8) -> Self {
    Self { red, green, blue }
  }

  pub fn random() -> Self {
    let mut r = rng();
    Color::new(
      r.random_range(0..=255),
      r.random_range(0..=255),
      r.random_range(0..=255),
    )
  }
}
