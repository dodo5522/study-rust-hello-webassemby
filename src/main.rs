mod browser;
mod engine;
mod graphics;

use crate::browser::accessor;
use crate::browser::context::context;
use crate::browser::wrapper::spawn_local;
use crate::engine::engine::load_image;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Deserialize)]
pub(crate) struct Rect {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

#[derive(Deserialize)]
pub(crate) struct Cell {
  pub frame: Rect,
  pub rotated: bool,
  pub trimmed: bool,
}

#[derive(Deserialize)]
pub(crate) struct Sheet {
  pub frames: HashMap<String, Cell>,
}

pub fn main() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  if let Ok(canvas) = context("canvas", "2d") {
    spawn_local(async move {
      let image = load_image("static/images/rhb.png")
        .await
        .expect("Cannot load image");
      let value = accessor::fetch_json("static/coordinates/rhb.json")
        .await
        .expect("rhn.json not found");
      let sheet = from_value::<Sheet>(value).expect("sheet not found on json");

      let mut frame = -1;
      let interval_callback = Closure::wrap(Box::new(move || {
        canvas.clear_rect(0.0, 0.0, 500.0, 500.0);
        frame = (frame + 1) % 8;

        let frame_name = format!("Run ({}).png", frame + 1);
        if let Some(cell) = sheet.frames.get(&frame_name) {
          canvas
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
              &image,
              cell.frame.x.into(),
              cell.frame.y.into(),
              cell.frame.w.into(),
              cell.frame.h.into(),
              300.0,
              300.0,
              cell.frame.w.into(),
              cell.frame.h.into(),
            )
            .expect("cannot draw image");
        }
      }) as Box<dyn FnMut()>);

      let window = browser::context::window().expect("Cannot get window object");
      let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
        interval_callback.as_ref().unchecked_ref(),
        50,
      );

      interval_callback.forget();
    });
  }

  Ok(())
}
