use ratatui::widgets::{ListState, TableState};

pub mod algorithm;
pub mod components;
pub mod constraints;
pub mod layout;
pub mod schedule;
pub mod tooltip;

pub enum Pane {
    Schedule,
    AlgorithmStats,
    Constraints,
}

pub struct UiState {
    pub schedule_table_state: TableState,
    pub stats_list_state: ListState,
}

impl UiState {
    pub fn new() -> UiState {
        let mut table_state = TableState::default();
        let mut stats_list_state = ListState::default().with_selected(Some(0));

        table_state.select_first();
        table_state.select_first_column();

        UiState {
            schedule_table_state: table_state,
            stats_list_state,
        }
    }
}
