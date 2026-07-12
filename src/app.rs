use std::{result::Result::Ok, time::Duration};

use anyhow::Result;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
};

use crate::{
    config::Config,
    constraints::constraint_store::{ConstraintStore, load_constraint_store_from_file},
    schedule::Schedule,
    ui::{UiState, layout::create_app_layout, schedule::render_schedule},
};

pub struct App {
    pub constraint_store: ConstraintStore,
    pub config: Config,
    pub schedule: Schedule,
    pub ui_state: UiState,
}

// TODO: Add docstrings
impl App {
    pub fn new() -> Result<App> {
        let config = Config::new()?;
        let constraint_store = load_constraint_store_from_file(&config.constraint_file_path)
            .expect("Could not load constraints from file. Please ensure the file exists");
        let schedule = Schedule::random(&constraint_store, config.random_seed, Some(0));

        Ok(App {
            constraint_store: constraint_store,
            config: config,
            schedule: schedule,
            ui_state: UiState::new(),
        })
    }

    pub fn run<B: Backend>(self: &mut Self, terminal: &mut Terminal<B>) -> Result<(), String> {
        loop {
            terminal
                .draw(|frame| {
                    let [schedule_block, _] = create_app_layout(frame);
                    render_schedule(
                        &mut self.schedule,
                        frame,
                        &schedule_block,
                        &mut self.ui_state.schedule_table_state,
                        &self.constraint_store,
                    );
                })
                .map_err(|e| e.to_string())?;

            match self._get_user_input() {
                Some(KeyCode::Char('q')) => {
                    break;
                }
                Some(KeyCode::Char('j')) => {
                    self.ui_state.schedule_table_state.select_next();
                }
                Some(KeyCode::Char('k')) => {
                    self.ui_state.schedule_table_state.select_previous();
                }
                Some(KeyCode::Char('h')) => {
                    self.ui_state.schedule_table_state.select_previous_column();
                }
                Some(KeyCode::Char('l')) => {
                    self.ui_state.schedule_table_state.select_next_column();
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// TODO: Add docstring
    pub fn _get_user_input(&self) -> Option<KeyCode> {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == event::KeyEventKind::Press {
                    return Some(key.code);
                }
            }
        }

        return None;
    }
}
