use anyhow::{Error, anyhow};
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

use crate::browser::accessor::{LoopClosure, now, request_animation_frame};
use crate::browser::context::context;
use crate::browser::wrapper::create_raf_closure;

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

const FRAME_GAP_MS: f32 = 1.0 / 60.0 * 1000.0;

#[async_trait]
pub(crate) trait Game {
  async fn initialize(&self) -> Result<Box<dyn Game>, Error>;
  fn update(&mut self);
  fn draw(&self, context: &CanvasRenderingContext2d);
}

pub(crate) struct GameLoop {
  last_frame: f64,        // 直前のフレームが リクエストされた時刻
  accumulated_delta: f32, // 最後に描画してから累積した差分時間
}

impl GameLoop {
  pub(crate) async fn start(game: impl Game + 'static) -> Result<(), Error> {
    let mut game = game.initialize().await?;
    let mut game_loop = GameLoop {
      last_frame: now()?,
      accumulated_delta: 0.0,
    };

    let f: SharedLoopClosure = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(create_raf_closure(move |perf: f64| {
      game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
      while game_loop.accumulated_delta > FRAME_GAP_MS {
        game.update();
        game_loop.accumulated_delta -= FRAME_GAP_MS;
      }
      game_loop.last_frame = perf;
      game.draw(&context("canvas", "2d").expect("Context should exist"));
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
