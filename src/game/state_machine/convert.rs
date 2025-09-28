use super::RedHatBoyStateMachine;
use super::state::RedHatBoyState;
use super::state::{Falling, Idle, JumpingInIdle, JumpingInRunning, KnockedOut, Running, Sliding};
use super::state::{
  FallingEndState, JumpingInIdleEndState, JumpingInRunningEndState, SlidingEndState,
};

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
