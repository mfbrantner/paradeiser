use clap::{arg, command, value_parser, ArgAction};

const DEFAULT_WORK_TIME: &str = "25";
const DEFAULT_SHORT_BREAK: &str = "5";
const DEFAULT_LONG_BREAK: &str = "10";

#[derive(Debug, Clone, Copy)]
pub struct Args {
    pub work_time: u64,
    pub short_break: u64,
    pub long_break: u64,
}

impl Args {
    pub fn parse() -> Self {
        let matches = command!()
            .arg(
                arg!(-w --work "Working time (minutes)")
                    .required(false)
                    .value_parser(value_parser!(u64))
                    .default_value(DEFAULT_WORK_TIME)
                    .action(ArgAction::Set)
                    .id("work")
            )
            .arg(
                arg!(-s --short "Short break time (minutes)")
                    .required(false)
                    .value_parser(value_parser!(u64))
                    .default_value(DEFAULT_SHORT_BREAK)
                    .action(ArgAction::Set)
                    .id("short_break")
            )
            .arg(
                arg!(-l --long "Long break time (minutes)")
                    .required(false)
                    .value_parser(value_parser!(u64))
                    .default_value(DEFAULT_LONG_BREAK)
                    .action(ArgAction::Set)
                    .id("long_break")
            )
            .get_matches();
        
        // unwrap is Ok here, according to clap docs
        let work_time = *matches.get_one::<u64>("work").unwrap();
        let short_break = *matches.get_one::<u64>("short_break").unwrap();
        let long_break = *matches.get_one::<u64>("long_break").unwrap();

        Self {
            work_time,
            short_break,
            long_break,
        }
    }
}
