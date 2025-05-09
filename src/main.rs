mod graphics;

use graphics::canvas::get_canvas_2d;
use graphics::color::Color;
use graphics::point::Point;
use graphics::triangle::draw_sierpinski;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::debug!("Hello, world!");

  if let Some(canvas) = get_canvas_2d("canvas") {
    let points = [
      Point::new(250.0, 0.0),
      Point::new(0.0, 500.0),
      Point::new(500.0, 500.0),
    ];
    let color = Color::random();

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

        if let Err(e) = canvas.draw_image_with_html_image_element(&img, 1.0, 1.0) {
          log::debug!("{:?}", e);
        }
      }

      draw_sierpinski(&canvas, &points, &color, 6, true);
    });
  }
}
