use crate::game::engine::browser::accessor::{LoopClosure, now, request_animation_frame};
use crate::game::engine::browser::context::context;
use crate::game::engine::browser::wrapper::create_raf_closure;
use crate::game::engine::renderer::Renderer;
use crate::game::engine::{Game, GameLoop};
use anyhow::{Error, anyhow};
use std::cell::RefCell;
use std::rc::Rc;

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

const FRAME_GAP_MS: f32 = 1.0 / 60.0 * 1000.0;

impl GameLoop {
  pub(crate) async fn start(game: impl Game + 'static) -> Result<(), Error> {
    let mut game = game.initialize().await?;
    let mut game_loop = GameLoop {
      last_frame: now()?,
      accumulated_delta: 0.0,
    };
    let renderer = Renderer::new(context("canvas", "2d")?);
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
