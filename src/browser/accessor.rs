use crate::browser::context::window;
use anyhow::anyhow;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys;

pub(crate) async fn fetch_with_str(resource: &str) -> Result<web_sys::Response, anyhow::Error> {
  JsFuture::from(window()?.fetch_with_str(resource))
    .await
    .map_err(|v| anyhow!("fetch('{}') failed", resource))?
    .dyn_into::<web_sys::Response>()
    .map_err(|v| anyhow!("Invalid fetch('{}') response", resource))
}

pub(crate) async fn fetch_json(path: &str) -> Result<JsValue, anyhow::Error> {
  let res = fetch_with_str(path).await?;
  JsFuture::from(res.json().map_err(|v| anyhow!("json() await error"))?)
    .await
    .map_err(|v| anyhow!("No json data"))
}
