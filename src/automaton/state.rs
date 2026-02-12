#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Created,
    Idle,
    Tested,
    Initialized,
    Connected,
    Ready,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Event {
    Start,
    Asap,
    Every30Seconds,
    Every3600Seconds,
    Pause,
    Resume,
    Stop,
    Error,
}
