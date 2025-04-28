mod graphics;

use graphics::*;
use web_sys::wasm_bindgen::JsCast;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Hello, world!");

    if let Some(canvas) = get_canvas_2d("canvas") {
        draw_triangle(canvas);
    }
}
