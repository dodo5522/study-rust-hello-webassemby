use anyhow::anyhow;
use wasm_bindgen::JsCast;
use web_sys::{Document, Window};

pub fn window() -> Result<Window, anyhow::Error> {
  web_sys::window().ok_or_else(|| anyhow!("No window object found"))
}

pub fn document() -> Result<Document, anyhow::Error> {
  window()?
    .document()
    .ok_or_else(|| anyhow!("No document object found"))
}

fn _canvas(element_id: &str) -> Result<web_sys::HtmlCanvasElement, anyhow::Error> {
  document()?
    .get_element_by_id(element_id)
    .ok_or_else(|| anyhow!("No element of ID '{}' found", element_id))?
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

pub fn canvas(
  element_id: &str,
  dimension: &str,
) -> Result<web_sys::CanvasRenderingContext2d, anyhow::Error> {
  _canvas(element_id)?
    .get_context(dimension)
    .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
    .ok_or_else(|| anyhow!("No 2d context found"))?
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .map_err(|element| {
      anyhow!(
        "Error converting {:#?} to CanvasRenderingContext2d",
        element,
      )
    })
}
