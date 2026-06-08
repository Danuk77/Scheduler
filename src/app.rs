use std::{result::Result::Ok, time::Duration};

use anyhow::Result;
use log::info;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
};

pub struct App {}

// TODO: Add docstrings
impl App {
    pub fn new() -> Result<App> {
        Ok(App {})
    }

    pub fn run<B: Backend>(self: &mut Self, terminal: &mut Terminal<B>) -> Result<(), String> {
        loop {
            terminal
                .draw(|_frame| info!("Hi"))
                .map_err(|e| e.to_string())?;

            match self._get_user_input() {
                Some(KeyCode::Char('q')) => {
                    break;
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
