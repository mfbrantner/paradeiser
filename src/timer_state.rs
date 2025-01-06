#[derive(Debug)]
pub enum TimerState {
    ShortBreak(String),
    LongBreak(String),
    Work(String)
}