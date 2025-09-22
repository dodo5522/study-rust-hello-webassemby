use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct SheetSize {
  pub w: u32,
  pub h: u32,
}

#[derive(Deserialize, Clone)]
pub struct SheetRect {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SheetCell {
  pub frame: SheetRect,
  pub rotated: bool,
  pub trimmed: bool,
  pub sprite_source_size: SheetRect,
  pub source_size: SheetSize,
}

#[derive(Deserialize, Clone)]
pub struct Sheet {
  pub frames: HashMap<String, SheetCell>,
}
