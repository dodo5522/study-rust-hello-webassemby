pub(crate) mod engine;

mod game;
mod sheet;

use crate::game::engine::{Game, Point};
use crate::game::sheet::Sheet;
use web_sys::HtmlImageElement;

pub struct WalkTheDog {
  image: Option<HtmlImageElement>,
  sheet: Option<Sheet>,
  frame: i32,
  position: Point,
}
