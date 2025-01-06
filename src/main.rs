mod pomodoro;
mod timer;
mod args;
mod app;
mod timer_state;
mod format;

use args::Args;
use app::App;

fn main() -> color_eyre::Result<()> {

    // parse args
    let args = Args::parse();
   
    // init color_eyre and ratatui
    color_eyre::install()?;
    let terminal = ratatui::init();

    // run app
    let result = App::new(args).run(terminal);

    // restore terminal
    ratatui::try_restore()?;

    // return result
    result
}
