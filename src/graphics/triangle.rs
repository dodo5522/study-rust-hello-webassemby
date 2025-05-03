use crate::graphics::color::Color;
use crate::graphics::point::Point;

fn get_center(first: &Point, second: &Point) -> Point {
  Point::new((first.x + second.x) / 2.0, (first.y + second.y) / 2.0)
}

pub fn draw_triangle(
  context: &web_sys::CanvasRenderingContext2d,
  points: &[Point; 3],
  color: &Color,
  fill: bool,
) {
  let top = &points[0];
  let bottom_left = &points[1];
  let bottom_right = &points[2];
  let color_str = format!("rgb({}, {}, {})", color.red, color.green, color.blue);

  context.move_to(top.x, top.y);

  context.set_stroke_style_str(&color_str);
  context.begin_path();
  context.line_to(bottom_left.x, bottom_left.y);
  context.line_to(bottom_right.x, bottom_right.y);
  context.line_to(top.x, top.y);
  context.close_path();

  context.stroke();

  if fill {
    context.set_fill_style_str(&color_str);
    context.fill();
  }
}

pub fn draw_sierpinski(
  context: &web_sys::CanvasRenderingContext2d,
  points: &[Point; 3],
  color: &Color,
  depth: u8,
  fill: bool,
) {
  draw_triangle(context, &points, color, fill);

  let depth = depth - 1;

  if depth > 0 {
    let top = &points[0];
    let bottom_left = &points[1];
    let bottom_right = &points[2];
    let next_color = Color::random();

    let points = [
      Point::new(top.x, top.y),
      get_center(&top, &bottom_left),
      get_center(&top, &bottom_right),
    ];
    draw_sierpinski(context, &points, &next_color, depth, fill);

    let points = [
      get_center(&top, &bottom_left),
      Point::new(bottom_left.x, bottom_left.y),
      get_center(&bottom_right, &bottom_left),
    ];
    draw_sierpinski(context, &points, &next_color, depth, fill);

    let points = [
      get_center(&top, &bottom_right),
      get_center(&bottom_right, &bottom_left),
      Point::new(bottom_right.x, bottom_right.y),
    ];
    draw_sierpinski(context, &points, &next_color, depth, fill);
  }
}
