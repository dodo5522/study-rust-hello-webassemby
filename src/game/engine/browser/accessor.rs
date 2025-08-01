use anyhow::{Error, anyhow};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Response};

use crate::game::engine::browser::{LoopClosure, window};

pub async fn fetch_with_str(resource: &str) -> Result<Response, Error> {
  JsFuture::from(window()?.fetch_with_str(resource))
    .await
    .map_err(|v| anyhow!("fetch('{}') failed", resource))?
    .dyn_into::<web_sys::Response>()
    .map_err(|v| anyhow!("Invalid fetch('{}') response", resource))
}

pub async fn fetch_json(path: &str) -> Result<JsValue, Error> {
  let res = fetch_with_str(path).await?;
  JsFuture::from(res.json().map_err(|v| anyhow!("json() await error"))?)
    .await
    .map_err(|v| anyhow!("No json data"))
}

pub fn new_image() -> Result<HtmlImageElement, Error> {
  HtmlImageElement::new().map_err(|v| anyhow!("Could not create HtmlImageElement: {:#?}", v))
}

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32, Error> {
  window()?
    .request_animation_frame(callback.as_ref().unchecked_ref())
    .map_err(|v| anyhow!("request_animation_frame() failed"))
}

pub fn now() -> Result<f64, Error> {
  Ok(
    window()?
      .performance()
      .ok_or(anyhow!("performance() not defined"))?
      .now(),
  )
}
