use anyhow::{Error, anyhow};
use futures::channel::mpsc::{UnboundedReceiver, unbounded};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use super::browser;
use super::{EngineLoop, Game, KeyPress, KeyState, Renderer, Size};

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;

const FRAME_GAP_MS: f32 = 1.0 / 60.0 * 1000.0;
const CANVAS_ID: &str = "canvas";
const CANVAS_DIMENSION: &str = "2d";

impl EngineLoop {
  pub async fn start(game: impl Game + 'static) -> Result<(), Error> {
    let mut key_receiver = prepare_input()?;
    let mut game = game.initialize().await?;
    let mut game_loop = EngineLoop {
      last_frame: browser::now()?,
      accumulated_delta: 0.0,
    };
    let renderer = Renderer::new(browser::canvas(CANVAS_ID, CANVAS_DIMENSION)?);
    let f: SharedLoopClosure = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut state = KeyState::new();

    *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
      process_input(&mut state, &mut key_receiver);
      game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
      while game_loop.accumulated_delta > FRAME_GAP_MS {
        game.update(&state);
        game_loop.accumulated_delta -= FRAME_GAP_MS;
      }
      game_loop.last_frame = perf;
      game.draw(&renderer);
      let _ = browser::request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    browser::request_animation_frame(
      g.borrow()
        .as_ref()
        .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
    )?;
    Ok(())
  }
}

fn prepare_input() -> Result<UnboundedReceiver<KeyPress>, Error> {
  let (key_sender, key_receiver) = unbounded::<KeyPress>();
  let key_down_sender = Rc::new(RefCell::new(key_sender));
  let key_up_sender = Rc::clone(&key_down_sender);

  let on_key_down = browser::closure_wrap(Box::new(move |key_code: KeyboardEvent| {
    let _ = key_down_sender
      .borrow_mut()
      .start_send(KeyPress::KeyDown(key_code));
  }) as Box<dyn FnMut(KeyboardEvent)>);

  let on_key_up = browser::closure_wrap(Box::new(move |key_code: KeyboardEvent| {
    let _ = key_up_sender
      .borrow_mut()
      .start_send(KeyPress::KeyUp(key_code));
  }) as Box<dyn FnMut(KeyboardEvent)>);

  let w = browser::window()?;
  w.set_onkeydown(Some(on_key_down.as_ref().unchecked_ref()));
  w.set_onkeyup(Some(on_key_up.as_ref().unchecked_ref()));

  on_key_down.forget();
  on_key_up.forget();
  Ok(key_receiver)
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

pub fn get_canvas_size() -> Result<Size, Error> {
  let canvas = browser::canvas(CANVAS_ID, CANVAS_DIMENSION)?
    .canvas()
    .expect("Cannot get canvas width");

  Ok(Size {
    width: canvas.width(),
    height: canvas.height(),
  })
}
