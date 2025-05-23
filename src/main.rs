mod browser;
mod graphics;

use crate::browser::accessor;
use crate::browser::context::context;
use crate::browser::future::spawn_local;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Deserialize)]
pub(crate) struct Rect {
  pub(crate) x: u32,
  pub(crate) y: u32,
  pub(crate) w: u32,
  pub(crate) h: u32,
}

#[derive(Deserialize)]
pub(crate) struct Cell {
  pub(crate) frame: Rect,
  pub(crate) rotated: bool,
  pub(crate) trimmed: bool,
}

#[derive(Deserialize)]
pub(crate) struct Sheet {
  pub(crate) frames: HashMap<String, Cell>,
}

pub fn main() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());

  if let Ok(canvas) = context("canvas", "2d") {
    spawn_local(async move {
      let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
      let success_tx = Rc::new(Mutex::new(Some(success_tx)));
      let error_tx = Rc::clone(&success_tx);

      let callback_image_on_load = Closure::once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
          if let Err(e) = success_tx.send(Ok(())) {
            log::debug!("{:?}", e);
          }
        }
      });
      let callback_image_on_error = Closure::once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
          if let Err(e) = error_tx.send(Err(err)) {
            log::debug!("{:?}", e);
          }
        }
      });

      let value = accessor::fetch_json("static/coordinates/rhb.json")
        .await
        .expect("rhn.json not found");
      let sheet = from_value::<Sheet>(value).expect("sheet not found on json");

      let image_element = web_sys::HtmlImageElement::new().expect("Image element creation failed");
      image_element.set_onload(Some(callback_image_on_load.as_ref().unchecked_ref()));
      image_element.set_onerror(Some(callback_image_on_error.as_ref().unchecked_ref()));
      image_element.set_src("static/images/rhb.png");

      let mut frame = -1;
      let interval_callback = Closure::wrap(Box::new(move || {
        canvas.clear_rect(0.0, 0.0, 500.0, 500.0);
        frame = (frame + 1) % 8;

        let frame_name = format!("Run ({}).png", frame + 1);
        if let Some(cell) = sheet.frames.get(&frame_name) {
          canvas
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
              &image_element,
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
      success_rx.await.ok();
    });
  }

  Ok(())
}
