use crate::game::engine::browser::accessor::{LoopClosure, now, request_animation_frame};
use crate::game::engine::browser::context::{canvas, window};
use crate::game::engine::browser::wrapper::{closure_wrap, create_raf_closure};
use crate::game::engine::renderer::Renderer;
use crate::game::engine::{Game, GameLoop};
use anyhow::{Error, anyhow};
use futures::channel::mpsc::{UnboundedReceiver, unbounded};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

const FRAME_GAP_MS: f32 = 1.0 / 60.0 * 1000.0;

impl GameLoop {
  pub(crate) async fn start(game: impl Game + 'static) -> Result<(), Error> {
    let mut game = game.initialize().await?;
    let mut game_loop = GameLoop {
      last_frame: now()?,
      accumulated_delta: 0.0,
    };
    let renderer = Renderer::new(canvas("canvas", "2d")?);
    let f: SharedLoopClosure = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(create_raf_closure(move |perf: f64| {
      game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
      while game_loop.accumulated_delta > FRAME_GAP_MS {
        game.update();
        game_loop.accumulated_delta -= FRAME_GAP_MS;
      }
      game_loop.last_frame = perf;
      game.draw(&renderer);
      let _ = request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(
      g.borrow()
        .as_ref()
        .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
    )?;
    Ok(())
  }
}

enum KeyPress {
  KeyDown(KeyboardEvent),
  KeyUp(KeyboardEvent),
}

fn prepare_input() -> Result<UnboundedReceiver<KeyPress>, Error> {
  let (key_sender, key_receiver) = unbounded::<KeyPress>();
  let key_down_sender = Rc::new(RefCell::new(key_sender));
  let key_up_sender = Rc::clone(&key_down_sender);

  let on_key_down = closure_wrap(Box::new(move |key_code: KeyboardEvent| {
    let _ = key_down_sender
      .borrow_mut()
      .start_send(KeyPress::KeyDown(key_code));
  }) as Box<dyn FnMut(KeyboardEvent)>);

  let on_key_up = closure_wrap(Box::new(move |key_code: KeyboardEvent| {
    let _ = key_up_sender
      .borrow_mut()
      .start_send(KeyPress::KeyUp(key_code));
  }) as Box<dyn FnMut(KeyboardEvent)>);

  let w = window()?;
  w.set_onkeydown(Some(on_key_down.as_ref().unchecked_ref()));
  w.set_onkeyup(Some(on_key_up.as_ref().unchecked_ref()));

  on_key_down.forget();
  on_key_up.forget();
  Ok(key_receiver)
}

struct KeyState {
  pressed_keys: HashMap<String, KeyboardEvent>,
}

fn process_input(state: &mut KeyState, key_receiver: &mut UnboundedReceiver<KeyPress>) {
  loop {
    match key_receiver.try_next() {
      Ok(Some(event)) => match event {
        KeyPress::KeyDown(event) => {
          state.set_pressed(&event.code(), event);
        }
        KeyPress::KeyUp(event) => {
          state.set_released(&event.code());
        }
      },
      _ => break,
    }
  }
}

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
