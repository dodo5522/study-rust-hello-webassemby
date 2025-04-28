use web_sys::wasm_bindgen::JsCast;

fn init() {
    wasm_logger::init(wasm_logger::Config::default());
}

fn draw_triangle() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id("canvas") {
                if let Ok(canvas) = element.dyn_into::<web_sys::HtmlCanvasElement>() {
                    if let Ok(Some(context)) = canvas.get_context("2d") {
                        if let Ok(context) = context.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                            context.move_to(200.0, 0.0);
                            context.begin_path();
                            context.line_to(0.0, 400.0); // bottom left of triangle context.line_to(600.0, 600.0); // bottom right of triangle context.line_to(300.0, 0.0); // back to top of triangle context.close_path();
                            context.line_to(400.0, 400.0); // bottom right of triangle context.line_to(300.0, 0.0); // back to top of triangle context.close_path();
                            context.line_to(200.0, 0.0);
                            context.close_path();
                            context.stroke();
                            context.fill();
                        }
                    }
                }
            }
        }
    }
}

pub fn main() {
    log::debug!("Hello, world!");

    init();
    draw_triangle();
}
