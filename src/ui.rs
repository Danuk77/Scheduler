use ratatui::widgets::TableState;

pub mod components;
pub mod layout;
pub mod schedule;

pub struct UiState {
    pub schedule_table_state: TableState,
}

impl UiState {
    pub fn new() -> UiState {
        let mut table_state = TableState::default();

        table_state.select_first();
        table_state.select_first_column();
        
        UiState {
            schedule_table_state: table_state,
        }
    }
}
