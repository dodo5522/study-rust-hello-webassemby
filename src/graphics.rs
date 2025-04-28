use web_sys::wasm_bindgen::JsCast;

pub fn get_canvas_2d(element_id: &str) -> Option<web_sys::CanvasRenderingContext2d> {
  let window = web_sys::window()?;
  let document = window.document()?;
  let element = document.get_element_by_id(element_id)?;
  let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
  let context = canvas.get_context("2d").ok().ok_or(()).unwrap()?;
  context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()
}

pub fn draw_triangle(context: web_sys::CanvasRenderingContext2d) {
  context.move_to(200.0, 0.0);
  context.begin_path();
  context.line_to(0.0, 400.0);
  context.line_to(400.0, 400.0);
  context.line_to(200.0, 0.0);
  context.close_path();
  context.stroke();
  context.fill();
}
