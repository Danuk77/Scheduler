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
    ui::{
        UiState, algorithm::render_algorithm_stats, config::render_config,
        constraints::render_constraints, input_handler::handle_user_input,
        layout::create_app_layout, schedule::render_schedule, tooltip::render_tooltip,
    },
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
                    let layout = create_app_layout(frame);
                    render_schedule(
                        &mut self.schedule,
                        frame,
                        &layout.schedule_block,
                        &self.constraint_store,
                        &mut self.ui_state,
                    );
                    render_tooltip(self.ui_state.selected_pane, frame, &layout.tooltip_block);
                    render_algorithm_stats(
                        frame,
                        &layout.algorithm_stats_block,
                        None,
                        &mut self.ui_state,
                    );
                    render_config(&self.config, frame, &layout.config, &mut self.ui_state);
                    render_constraints(frame, &layout.constraints_block, &mut self.ui_state);
                })
                .map_err(|e| e.to_string())?;

            if let Some(key_event) = self._get_user_input() {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => {
                        break;
                    }
                    (KeyCode::Tab, KeyModifiers::NONE) => {
                        self.ui_state.select_next_pane();
                    }
                    (KeyCode::BackTab, _) => {
                        self.ui_state.select_previous_pane();
                    }
                    _ => {
                        handle_user_input(&mut self.ui_state, key_event.code, key_event.modifiers);
                    }
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
