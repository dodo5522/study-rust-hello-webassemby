use super::RedHatBoyStateMachine;
use super::context::RedHatBoyContext;
use super::event::Event;

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
