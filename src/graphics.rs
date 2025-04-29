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

fn get_center(first: &Point, second: &Point) -> Point {
  Point::new((first.x + second.x) / 2.0, (first.y + second.y) / 2.0)
}

pub fn draw_sierpinski(
  context: &web_sys::CanvasRenderingContext2d,
  points: &[Point; 3],
  depth: u8,
  fill: bool,
) {
  draw_triangle(context, &points, fill);

  let depth = depth - 1;

  if depth > 0 {
    let top = &points[0];
    let bottom_left = &points[1];
    let bottom_right = &points[2];

    let points = [
      Point::new(top.x, top.y),
      get_center(&top, &bottom_left),
      get_center(&top, &bottom_right),
    ];
    draw_sierpinski(context, &points, depth, fill);

    let points = [
      get_center(&top, &bottom_left),
      Point::new(bottom_left.x, bottom_left.y),
      get_center(&bottom_right, &bottom_left),
    ];
    draw_sierpinski(context, &points, depth, fill);

    let points = [
      get_center(&top, &bottom_right),
      get_center(&bottom_right, &bottom_left),
      Point::new(bottom_right.x, bottom_right.y),
    ];
    draw_sierpinski(context, &points, depth, fill);
  }
}
