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

use crate::game::state::{
  Falling, Jumping, RedHatBoyContext, RedHatBoyState, RedHatBoyStateMachine, Running, Sliding,
};

enum Event {
  Run,
  Jump,
  Slide,
  Crash,
}

impl RedHatBoyStateMachine {
  fn transition(self, event: Event) -> Self {
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
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Running>) -> Self {
    RedHatBoyStateMachine::Running(state)
  }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Jumping>) -> Self {
    RedHatBoyStateMachine::Jumping(state)
  }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Sliding>) -> Self {
    RedHatBoyStateMachine::Sliding(state)
  }
}

impl From<RedHatBoyState<Falling>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Falling>) -> Self {
    RedHatBoyStateMachine::Falling(state)
  }
}
