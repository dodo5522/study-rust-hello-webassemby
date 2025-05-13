use anyhow::anyhow;
use wasm_bindgen::JsCast;
use web_sys::{Document, Window};

pub(crate) fn window() -> Result<Window, anyhow::Error> {
  web_sys::window().ok_or_else(|| anyhow!("Cannot get window object"))
}

pub(crate) fn document() -> Result<Document, anyhow::Error> {
  window()?
    .document()
    .ok_or_else(|| anyhow!("Cannot get document object"))
}

pub fn canvas_2d(element_id: &str) -> Option<web_sys::CanvasRenderingContext2d> {
  let window = web_sys::window()?;
  let document = window.document()?;
  let element = document.get_element_by_id(element_id)?;
  let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
  let context = canvas.get_context("2d").ok().ok_or(()).unwrap()?;
  context.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()
}
