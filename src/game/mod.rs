mod engine;
pub type GameLoop = engine::EngineLoop;
pub use engine::spawn_local;

mod sheet;
mod walk_the_dog;

use web_sys::HtmlImageElement;

pub struct WalkTheDog {
  image: Option<HtmlImageElement>,
  sheet: Option<sheet::Sheet>,
  frame: i32,
  position: engine::Point,
}
