// use chrono::Duration;
use std::time::Duration;

pub struct App {
    pub current_pomodoro: Duration,
    pub current_break: Duration,
    pub past_pomodoros: u32,
    pub state: State,
}

pub enum State {
    Running,
    NextBreak(Duration),
    Paused,
}

impl App {
    pub fn new() -> App {
        App {
            current_pomodoro: Duration::from_secs(0),
            current_break: Duration::from_secs(0),
            // delay_break: Duration::zero(),
            past_pomodoros: 0,
            state: State::Running,
        }
    }
}
