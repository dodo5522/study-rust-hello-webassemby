mod engine;
pub type GameLoop = engine::EngineLoop;
pub use engine::spawn_local;

mod red_hat_boy;

mod sheet;
pub use sheet::Sheet;

mod state_machine;

mod walk_the_dog;
pub use walk_the_dog::WalkTheDog;
