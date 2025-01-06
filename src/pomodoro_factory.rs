use crate::args::Args;
use crate::pomodoro::{Pomodoro, Break};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct PomodoroFactory {
    work_time: Duration,
    short_break: Break,
    long_break: Break,
}

impl PomodoroFactory {
    
    pub fn new(args: &Args) -> Self {
        Self {
            work_time: Duration::from_secs(args.work_time * 60),
            short_break: Break::Short(Duration::from_secs(args.short_break * 60)),
            long_break: Break::Long(Duration::from_secs(args.long_break * 60)),
        }
    }

    pub fn short(&self) -> Pomodoro {
        Pomodoro {
            work_time: self.work_time,
            break_time: self.short_break,
        }
    }

    pub fn long(&self) -> Pomodoro {
        Pomodoro {
            work_time: self.work_time,
            break_time: self.long_break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compound() {
        let work = 25;
        let short_break = 5;
        let long_break = 10;

        let args = Args {
            work_time: work,
            short_break,
            long_break
        };

        let factory = PomodoroFactory::new(&args);

        let short_pomodoro = factory.short();
        let long_pomodoro = factory.long();

        assert_eq!(short_pomodoro.work_time, Duration::from_secs(work * 60));
        assert_eq!(short_pomodoro.break_time, Break::Short(Duration::from_secs(short_break * 60)));

        assert_eq!(long_pomodoro.work_time, Duration::from_secs(work * 60));
        assert_eq!(long_pomodoro.break_time, Break::Long(Duration::from_secs(long_break * 60)));
    }

    #[test]
    fn test_short() {
        let work = 25;
        let short_break = 5;
        let long_break = 10;

        let args = Args {
            work_time: work,
            short_break,
            long_break
        };

        let factory = PomodoroFactory::new(&args);

        let short_pomodoro = factory.short();

        assert_eq!(short_pomodoro.work_time, Duration::from_secs(work * 60));
        assert_eq!(short_pomodoro.break_time, Break::Short(Duration::from_secs(short_break * 60)));
    }

    #[test]
    fn test_long() {
        let work = 25;
        let short_break = 5;
        let long_break = 10;

        let args = Args {
            work_time: work,
            short_break,
            long_break
        };

        let factory = PomodoroFactory::new(&args);

        let long_pomodoro = factory.long();

        assert_eq!(long_pomodoro.work_time, Duration::from_secs(work * 60));
        assert_eq!(long_pomodoro.break_time, Break::Long(Duration::from_secs(long_break * 60)));
    }

    #[test]
    fn test_different_times() {
        let work = 30;
        let short_break = 10;
        let long_break = 15;

        let args = Args {
            work_time: work,
            short_break,
            long_break
        };

        let factory = PomodoroFactory::new(&args);

        let short_pomodoro = factory.short();
        let long_pomodoro = factory.long();

        assert_eq!(short_pomodoro.work_time, Duration::from_secs(work * 60));
        assert_eq!(short_pomodoro.break_time, Break::Short(Duration::from_secs(short_break * 60)));

        assert_eq!(long_pomodoro.work_time, Duration::from_secs(work * 60));
        assert_eq!(long_pomodoro.break_time, Break::Long(Duration::from_secs(long_break * 60)));
    }

    #[test]
    fn test_zero_times() {
        let work = 0;
        let short_break = 0;
        let long_break = 0;

        let args = Args {
            work_time: work,
            short_break,
            long_break
        };

        let factory = PomodoroFactory::new(&args);

        let short_pomodoro = factory.short();
        let long_pomodoro = factory.long();

        assert_eq!(short_pomodoro.work_time, Duration::from_secs(0));
        assert_eq!(short_pomodoro.break_time, Break::Short(Duration::from_secs(0)));

        assert_eq!(long_pomodoro.work_time, Duration::from_secs(0));
        assert_eq!(long_pomodoro.break_time, Break::Long(Duration::from_secs(0)));
    }

}