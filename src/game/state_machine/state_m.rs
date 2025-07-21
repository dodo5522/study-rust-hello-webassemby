// | Previous    | Event    | Transitioned | What to do          |
// |-------------|----------|--------------|---------------------|
// | Idle        | run      | Running      | Increase x velocity |
// | Running     | jump     | Jumping      |                     |
// | Running     | slide    | Sliding      |                     |
// | Running     | crash    | Falling      |                     |
// | Jumping     | land     | Running      |                     |
// | Jumping     | crash    | Falling      |                     |
// | Sliding     | stand up | Running      |                     |
// | Sliding     | crash    | Falling      |                     |
// | Falling     | end      | Knocked Out  |                     |
// | Knocked Out |          |              |                     |

use super::RedHatBoyContext;
use super::state;
use super::{Falling, Idle, Jumping, KnockedOut, Running, Sliding};

#[derive(Copy, Clone)]
pub enum RedHatBoyStateMachine {
  Idle(state::RedHatBoyState<Idle>),
  Running(state::RedHatBoyState<Running>),
  Jumping(state::RedHatBoyState<Jumping>),
  Sliding(state::RedHatBoyState<Sliding>),
  Falling(state::RedHatBoyState<Falling>),
  KnockedOut(state::RedHatBoyState<KnockedOut>),
}

pub enum Event {
  Run,
  Jump,
  Slide,
  Crash,
}

impl RedHatBoyStateMachine {
  pub fn transition(self, event: Event) -> Self {
    match (&self, event) {
      (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
      (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
      (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
      (RedHatBoyStateMachine::Running(state), Event::Crash) => state.crash().into(),
      _ => self,
    }
  }

  pub fn context(&self) -> &RedHatBoyContext {
    match self {
      RedHatBoyStateMachine::Idle(state) => state.context(),
      RedHatBoyStateMachine::Running(state) => state.context(),
      RedHatBoyStateMachine::Jumping(state) => state.context(),
      RedHatBoyStateMachine::Sliding(state) => state.context(),
      RedHatBoyStateMachine::Falling(state) => state.context(),
      RedHatBoyStateMachine::KnockedOut(state) => state.context(),
    }
  }

  pub fn frame_name(&self) -> &str {
    match self {
      RedHatBoyStateMachine::Idle(state) => state.frame_name(),
      RedHatBoyStateMachine::Running(state) => state.frame_name(),
      RedHatBoyStateMachine::Jumping(state) => state.frame_name(),
      RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
      RedHatBoyStateMachine::Falling(state) => state.frame_name(),
      RedHatBoyStateMachine::KnockedOut(state) => state.frame_name(),
    }
  }

  pub fn update(mut self) -> Self {
    match self {
      RedHatBoyStateMachine::Idle(mut state) => {
        state.update();
        RedHatBoyStateMachine::Idle(state)
      }
      RedHatBoyStateMachine::Running(mut state) => {
        state.update();
        RedHatBoyStateMachine::Running(state)
      }
      RedHatBoyStateMachine::Jumping(mut state) => {
        state.update();
        RedHatBoyStateMachine::Jumping(state)
      }
      RedHatBoyStateMachine::Sliding(mut state) => {
        state.update();
        RedHatBoyStateMachine::Sliding(state)
      }
      RedHatBoyStateMachine::Falling(mut state) => {
        state.update();
        RedHatBoyStateMachine::Falling(state)
      }
      RedHatBoyStateMachine::KnockedOut(mut state) => {
        state.update();
        RedHatBoyStateMachine::KnockedOut(state)
      }
    }
  }
}

impl From<state::RedHatBoyState<Running>> for RedHatBoyStateMachine {
  fn from(state: state::RedHatBoyState<Running>) -> Self {
    RedHatBoyStateMachine::Running(state)
  }
}

impl From<state::RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
  fn from(state: state::RedHatBoyState<Jumping>) -> Self {
    RedHatBoyStateMachine::Jumping(state)
  }
}

impl From<state::RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
  fn from(state: state::RedHatBoyState<Sliding>) -> Self {
    RedHatBoyStateMachine::Sliding(state)
  }
}

impl From<state::RedHatBoyState<Falling>> for RedHatBoyStateMachine {
  fn from(state: state::RedHatBoyState<Falling>) -> Self {
    RedHatBoyStateMachine::Falling(state)
  }
}
