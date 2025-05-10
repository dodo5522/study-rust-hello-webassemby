use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys;

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

pub async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
  let window = web_sys::window().ok_or(JsValue::from_str("no global window found"))?;
  let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;
  let resp: web_sys::Response = resp_value.dyn_into()?;
  wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}
