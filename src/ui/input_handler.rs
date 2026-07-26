use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::ui::{Pane, UiState};

pub fn handle_user_input(ui_state: &mut UiState, key_event: KeyCode, key_modifier: KeyModifiers) {
    match ui_state.selected_pane {
        Pane::Constraints => {}
        Pane::Schedule => _handle_schedule_inputs(ui_state, key_event, key_modifier),
        Pane::AlgorithmStats => _handle_algorithm_stats_inputs(ui_state, key_event, key_modifier),
        Pane::Config => _handle_config_stats_inputs(ui_state, key_event, key_modifier),
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

fn _handle_algorithm_stats_inputs(
    ui_state: &mut UiState,
    key_event: KeyCode,
    key_modifier: KeyModifiers,
) {
    const MULTIPLE_SCROLL_OFFSET: u16 = 6;
    match (key_event, key_modifier) {
        (KeyCode::Char('j'), KeyModifiers::NONE) => {
            ui_state.stats_list_state.select_next();
        }
        (KeyCode::Char('k'), KeyModifiers::NONE) => {
            ui_state.stats_list_state.select_previous();
        }
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            ui_state
                .stats_list_state
                .scroll_down_by(MULTIPLE_SCROLL_OFFSET);
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            ui_state
                .stats_list_state
                .scroll_up_by(MULTIPLE_SCROLL_OFFSET);
        }
        _ => {}
    }
}

fn _handle_config_stats_inputs(
    ui_state: &mut UiState,
    key_event: KeyCode,
    key_modifier: KeyModifiers,
) {
    const MULTIPLE_SCROLL_OFFSET: u16 = 6;
    match (key_event, key_modifier) {
        (KeyCode::Char('j'), KeyModifiers::NONE) => {
            ui_state.config_list_state.select_next();
        }
        (KeyCode::Char('k'), KeyModifiers::NONE) => {
            ui_state.config_list_state.select_previous();
        }
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            ui_state
                .config_list_state
                .scroll_down_by(MULTIPLE_SCROLL_OFFSET);
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            ui_state
                .config_list_state
                .scroll_up_by(MULTIPLE_SCROLL_OFFSET);
        }
        _ => {}
    }
}
