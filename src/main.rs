mod config;
mod constraints;
mod global_search;
mod hill_climber;
mod random;
mod schedule;
mod stats;

mod app;
mod ui;
mod terminal;

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: If the algorithm should be runnable in a headless way, then we can use the env_logger
    //env_logger::init();
    let mut terminal = terminal::setup_terminal()?;
    let mut app = App::new()?;
    app.run(&mut terminal)?;
    terminal::restore_terminal(terminal)?;
    Ok(())
}
