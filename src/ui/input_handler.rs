use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::ui::{Pane, UiState};

pub fn handle_user_input(ui_state: &mut UiState, key_event: KeyCode, key_modifier: KeyModifiers) {
    match ui_state.selected_pane {
        Pane::Constraints => {}
        Pane::Schedule => _handle_schedule_inputs(ui_state, key_event, key_modifier),
        Pane::AlgorithmStats => {}
        _ => {}
    }
}

fn _handle_schedule_inputs(ui_state: &mut UiState, key_event: KeyCode, key_modifier: KeyModifiers) {
    match (key_event, key_modifier) {
        (KeyCode::Char('j'), KeyModifiers::NONE) => {
            ui_state.schedule_table_state.select_next();
        }
        (KeyCode::Char('k'), KeyModifiers::NONE) => {
            ui_state.schedule_table_state.select_previous();
        }
        (KeyCode::Char('h'), KeyModifiers::NONE) => {
            ui_state.schedule_table_state.select_previous_column();
        }
        (KeyCode::Char('l'), KeyModifiers::NONE) => {
            ui_state.schedule_table_state.select_next_column();
        }
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            ui_state.schedule_table_state.scroll_down_by(15);
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            ui_state.schedule_table_state.scroll_up_by(15);
        }
        _ => {}
    }
}
