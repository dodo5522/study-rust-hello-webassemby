use std::collections::HashMap;
use web_sys::KeyboardEvent;

use crate::game::engine::KeyState;

impl KeyState {
  pub fn new() -> Self {
    KeyState {
      pressed_keys: HashMap::new(),
    }
  }

  pub fn is_pressed(&self, key: &str) -> bool {
    self.pressed_keys.contains_key(key.into())
  }

  pub fn set_pressed(&mut self, key: &str, event: KeyboardEvent) {
    self.pressed_keys.insert(key.into(), event);
  }

  pub fn set_released(&mut self, key: &str) {
    self.pressed_keys.remove(key.into());
  }
}
