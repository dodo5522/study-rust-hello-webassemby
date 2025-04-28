use web_sys::wasm_bindgen::JsCast;

pub struct Point {
  pub(crate) x: f64,
  y: f64,
}

impl Point {
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }
}

pub fn get_canvas_2d(element_id: &str) -> Option<web_sys::CanvasRenderingContext2d> {
  let window = web_sys::window()?;
  let document = window.document()?;
  let element = document.get_element_by_id(element_id)?;
  let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
  let context = canvas.get_context("2d").ok().ok_or(()).unwrap()?;
  context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()
}

pub fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: &[Point; 3], fill: bool) {
  let top = &points[0];
  let bottom_left = &points[1];
  let bottom_right = &points[2];

  context.move_to(top.x, top.y);

  context.begin_path();
  context.line_to(bottom_left.x, bottom_left.y);
  context.line_to(bottom_right.x, bottom_right.y);
  context.line_to(top.x, top.y);
  context.close_path();

  context.stroke();

  if fill {
    context.fill();
  }
}
