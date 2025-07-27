use super::engine;

mod context;
pub use context::RedHatBoyContext;

mod state;
pub use state::RedHatBoyStateIdle;

mod state_m;
pub use state_m::Event;
pub use state_m::RedHatBoyStateMachine;

#[derive(Copy, Clone)]
pub struct Idle;
#[derive(Copy, Clone)]
struct Running;
#[derive(Copy, Clone)]
struct Jumping;
#[derive(Copy, Clone)]
struct Sliding;
#[derive(Copy, Clone)]
struct Falling;
#[derive(Copy, Clone)]
struct KnockedOut;
