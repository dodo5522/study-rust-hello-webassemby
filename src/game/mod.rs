use web_sys::HtmlImageElement;

mod engine;
pub type GameLoop = engine::EngineLoop;
pub use engine::spawn_local;

mod state;
mod walk_the_dog;

pub struct WalkTheDog {
  image: Option<HtmlImageElement>,
  sheet: Option<state::Sheet>,
  frame: i32,
  position: engine::Point,
}
