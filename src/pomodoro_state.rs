use crate::args::Args;
use crate::pomodoro::Pomodoro;
use crate::pomodoro_factory::PomodoroFactory;

const INTERVAL_LEN: u32 = 3;

#[derive(Debug, Copy, Clone)]
pub struct PomodoroState {
    p_factory: PomodoroFactory,
    count: u32,
}

impl PomodoroState {
    pub fn new(args: &Args) -> Self {
        PomodoroState {
            p_factory: PomodoroFactory::new(args),
            count: 0,
        }
    }

    pub fn next(&mut self) -> Pomodoro {

        if self.count == 0 {
            self.count += 1;
            self.p_factory.short()
        }
        else if self.count % INTERVAL_LEN == 0 {
            self.count += 1;
            return self.p_factory.long();
        }
        else {
            self.count += 1;
            return self.p_factory.short();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::pomodoro::Break;

    use super::*;

    #[test]
    fn test_pomodoro_state() {
        let work = 25;
        let short_break = 5;
        let long_break = 10;

        let p_short = Pomodoro {
            work_time: Duration::from_secs(work * 60),
            break_time: Break::Short(Duration::from_secs(short_break * 60)),
        };
        let p_long = Pomodoro {
            work_time: Duration::from_secs(work * 60),
            break_time: Break::Long(Duration::from_secs(long_break * 60)),
        };

        let args = Args {
            work_time: work,
            short_break,
            long_break
        };

        let mut pomodoro_state = PomodoroState::new(&args);

        assert_eq!(pomodoro_state.count, 0);
        assert_eq!(pomodoro_state.next(), p_short);
        assert_eq!(pomodoro_state.count, 1);

        assert_eq!(pomodoro_state.next(), p_short);
        assert_eq!(pomodoro_state.count, 2);

        assert_eq!(pomodoro_state.next(), p_short);
        assert_eq!(pomodoro_state.count, 3);

        assert_eq!(pomodoro_state.next(), p_long);
        assert_eq!(pomodoro_state.count, 4);

        assert_eq!(pomodoro_state.next(), p_short);
        assert_eq!(pomodoro_state.count, 5);

    }
}