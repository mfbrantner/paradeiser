use crate::format::format_duration;
use crate::timer::Timer;
use crate::timer_state::TimerState;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Break {
    Short(Duration),
    Long(Duration),
}

impl Break {
    pub fn duration(&self) -> &Duration {
        match self {
            Break::Short(d) => d,
            Break::Long(d) => d,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pomodoro {
    pub work_time: Duration,
    pub break_time: Break,
}

impl Pomodoro {
    pub fn run(&self, t_state: Arc<Mutex<TimerState>>) {
        // work
        Timer::run(&self.work_time, |remaining| {
            //println!("[Work] Time remaining: {}", remaining.as_secs());
            {
                let mut state = t_state.lock().unwrap();
                *state = TimerState::Work(format_duration(&remaining));
            }
        });

        // break
        let dur = self.break_time.duration();

        Timer::run(dur, |remaining| {
            // println!("[{msg}] Time remaining: {}", remaining.as_secs());

            let mut state = t_state.lock().unwrap();
            *state = match self.break_time {
                Break::Short(_) => TimerState::ShortBreak(format_duration(&remaining)),
                Break::Long(_) => TimerState::LongBreak(format_duration(&remaining)),
            }
        });
    }
}
