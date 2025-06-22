use wasm_bindgen::closure::Closure;

mod accessor;
pub use accessor::*;
mod context;
pub use context::*;
mod wrapper;
pub use wrapper::*;

pub type LoopClosure = Closure<dyn FnMut(f64)>;
