use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct SheetRect {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

#[derive(Deserialize)]
pub(crate) struct SheetCell {
  pub frame: SheetRect,
  pub rotated: bool,
  pub trimmed: bool,
}

#[derive(Deserialize)]
pub(crate) struct Sheet {
  pub frames: HashMap<String, SheetCell>,
}
