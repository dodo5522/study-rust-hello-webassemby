use anyhow::{Error, anyhow};
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlImageElement;

use crate::game::engine::browser;

pub async fn load_image(source: &str) -> Result<HtmlImageElement, Error> {
  let image = browser::new_image()?;

  let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), Error>>();
  let success_tx = Rc::new(Mutex::new(Some(success_tx)));
  let error_tx = Rc::clone(&success_tx);

  let callback_image_on_load = Closure::once(move || {
    if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
      if let Err(e) = success_tx.send(Ok(())) {
        log::debug!("{:?}", e);
      }
    }
  });
  let callback_image_on_error: Closure<dyn FnMut(JsValue)> = Closure::once(move |err| {
    if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
      if let Err(e) = error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err))) {
        log::debug!("{:?}", e);
      }
    }
  });

  image.set_onload(Some(callback_image_on_load.as_ref().unchecked_ref()));
  image.set_onerror(Some(callback_image_on_error.as_ref().unchecked_ref()));
  image.set_src(source);

  success_rx.await??;
  Ok(image)
}
