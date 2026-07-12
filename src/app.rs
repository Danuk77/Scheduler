use std::{result::Result::Ok, time::Duration};

use anyhow::Result;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
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

            if let Some(key_event) = self._get_user_input() {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        break;
                    }
                    (KeyCode::Char('j'), KeyModifiers::NONE) => {
                        self.ui_state.schedule_table_state.select_next();
                    }
                    (KeyCode::Char('k'), KeyModifiers::NONE) => {
                        self.ui_state.schedule_table_state.select_previous();
                    }
                    (KeyCode::Char('h'), KeyModifiers::NONE) => {
                        self.ui_state.schedule_table_state.select_previous_column();
                    }
                    (KeyCode::Char('l'), KeyModifiers::NONE) => {
                        self.ui_state.schedule_table_state.select_next_column();
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.ui_state.schedule_table_state.scroll_down_by(10);
                    }
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        self.ui_state.schedule_table_state.scroll_up_by(10);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// TODO: Add docstring
    pub fn _get_user_input(&self) -> Option<KeyEvent> {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == event::KeyEventKind::Press {
                    return Some(key);
                }
            }
        }

        return None;
    }
}
