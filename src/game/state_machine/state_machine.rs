use super::RedHatBoyContext;
use super::state::{
  Falling, Idle, JumpingInIdle, JumpingInRunning, KnockedOut, RedHatBoyState, Running, Sliding,
};
use super::state::{
  FallingEndState, JumpingInIdleEndState, JumpingInRunningEndState, SlidingEndState,
};

#[derive(Copy, Clone)]
pub enum RedHatBoyStateMachine {
  Falling(RedHatBoyState<Falling>),
  Idle(RedHatBoyState<Idle>),
  Running(RedHatBoyState<Running>),
  JumpingInIdle(RedHatBoyState<JumpingInIdle>),
  JumpingInRunning(RedHatBoyState<JumpingInRunning>),
  Sliding(RedHatBoyState<Sliding>),
  KnockedOut(RedHatBoyState<KnockedOut>),
}

pub enum Event {
  Run,
  Jump,
  Slide,
  KnockOut,
  Stop,
  Update,
}

impl RedHatBoyStateMachine {
  pub fn transition(self, event: Event) -> Self {
    match (&self, event) {
      (RedHatBoyStateMachine::Falling(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::Idle(state), Event::Jump) => state.jump().into(),
      (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
      (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
      (RedHatBoyStateMachine::Running(state), Event::Stop) => state.stand().into(),
      (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
      (RedHatBoyStateMachine::Running(state), Event::KnockOut) => state.knock_out().into(),
      (RedHatBoyStateMachine::JumpingInIdle(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::JumpingInRunning(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::JumpingInRunning(state), Event::KnockOut) => state.knock_out().into(),
      (RedHatBoyStateMachine::KnockedOut(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::Sliding(state), Event::Update) => state.update().into(),
      (RedHatBoyStateMachine::Sliding(state), Event::KnockOut) => state.knock_out().into(),
      _ => self,
    }
  }

  pub fn context(&self) -> &RedHatBoyContext {
    match self {
      RedHatBoyStateMachine::Idle(state) => state.context(),
      RedHatBoyStateMachine::Running(state) => state.context(),
      RedHatBoyStateMachine::JumpingInIdle(state) => state.context(),
      RedHatBoyStateMachine::JumpingInRunning(state) => state.context(),
      RedHatBoyStateMachine::Sliding(state) => state.context(),
      RedHatBoyStateMachine::Falling(state) => state.context(),
      RedHatBoyStateMachine::KnockedOut(state) => state.context(),
    }
  }

  pub fn frame_name(&self) -> &str {
    match self {
      RedHatBoyStateMachine::Idle(state) => state.frame_name(),
      RedHatBoyStateMachine::Running(state) => state.frame_name(),
      RedHatBoyStateMachine::JumpingInIdle(state) => state.frame_name(),
      RedHatBoyStateMachine::JumpingInRunning(state) => state.frame_name(),
      RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
      RedHatBoyStateMachine::Falling(state) => state.frame_name(),
      RedHatBoyStateMachine::KnockedOut(state) => state.frame_name(),
    }
  }

  pub fn update(self) -> Self {
    self.transition(Event::Update)
  }
}

impl From<RedHatBoyState<Falling>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Falling>) -> Self {
    RedHatBoyStateMachine::Falling(state)
  }
}

impl From<FallingEndState> for RedHatBoyStateMachine {
  fn from(end_state: FallingEndState) -> Self {
    match end_state {
      FallingEndState::Falling(state) => state.into(),
      FallingEndState::Complete(state) => state.into(),
    }
  }
}

impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Idle>) -> Self {
    RedHatBoyStateMachine::Idle(state)
  }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Running>) -> Self {
    RedHatBoyStateMachine::Running(state)
  }
}

impl From<RedHatBoyState<JumpingInIdle>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<JumpingInIdle>) -> Self {
    RedHatBoyStateMachine::JumpingInIdle(state)
  }
}

impl From<JumpingInIdleEndState> for RedHatBoyStateMachine {
  fn from(end_state: JumpingInIdleEndState) -> Self {
    match end_state {
      JumpingInIdleEndState::Jumping(state) => state.into(),
      JumpingInIdleEndState::Complete(state) => state.into(),
    }
  }
}

impl From<RedHatBoyState<JumpingInRunning>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<JumpingInRunning>) -> Self {
    RedHatBoyStateMachine::JumpingInRunning(state)
  }
}

impl From<JumpingInRunningEndState> for RedHatBoyStateMachine {
  fn from(end_state: JumpingInRunningEndState) -> Self {
    match end_state {
      JumpingInRunningEndState::Jumping(state) => state.into(),
      JumpingInRunningEndState::Complete(state) => state.into(),
    }
  }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<Sliding>) -> Self {
    RedHatBoyStateMachine::Sliding(state)
  }
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
  fn from(end_state: SlidingEndState) -> Self {
    match end_state {
      SlidingEndState::Sliding(state) => state.into(),
      SlidingEndState::Complete(state) => state.into(),
    }
  }
}

impl From<RedHatBoyState<KnockedOut>> for RedHatBoyStateMachine {
  fn from(state: RedHatBoyState<KnockedOut>) -> Self {
    RedHatBoyStateMachine::KnockedOut(state)
  }
}
