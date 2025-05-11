mod graphics;
mod reader;

use graphics::canvas::get_canvas_2d;
use reader::json;
use serde_wasm_bindgen::from_value;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::debug!("Hello, world!");

  if let Some(canvas) = get_canvas_2d("canvas") {
    wasm_bindgen_futures::spawn_local(async move {
      let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
      let success_tx = Rc::new(Mutex::new(Some(success_tx)));
      let error_tx = Rc::clone(&success_tx);

      if let Ok(img) = web_sys::HtmlImageElement::new() {
        let callback = Closure::once(move || {
          if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            if let Err(e) = success_tx.send(Ok(())) {
              log::debug!("{:?}", e);
            }
          }
        });
        let error_callback = Closure::once(move |err| {
          if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            if let Err(e) = error_tx.send(Err(err)) {
              log::debug!("{:?}", e);
            }
          }
        });

        img.set_onload(Some(callback.as_ref().unchecked_ref()));
        img.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

        img.set_src("static/images/rhb.png");
        success_rx.await.ok();

        if let Ok(json) = json::fetch_json("static/coordinates/rhb.json").await {
          if let Ok(sheet) = from_value::<reader::json::Sheet>(json) {
            if let Some(cell) = sheet.frames.get("Dead (1).png") {
              canvas
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                  &img,
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
          }
        }
      }
    });
  }
}
