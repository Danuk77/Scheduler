use ratatui::widgets::{ListState, TableState};

pub mod algorithm;
pub mod components;
pub mod config;
pub mod constraints;
pub mod input_handler;
pub mod layout;
pub mod schedule;
pub mod tooltip;

#[derive(Copy, Clone)]
pub enum Pane {
    Schedule,
    AlgorithmStats,
    Constraints,
    Config,
}

pub struct UiState {
    pub selected_pane: Pane,
    pub schedule_table_state: TableState,
    pub stats_list_state: ListState,
    pub config_list_state: ListState,
}

impl UiState {
    pub fn new() -> UiState {
        let mut table_state = TableState::default();
        let mut stats_list_state = ListState::default().with_selected(Some(0));
        let mut config_list_state = ListState::default().with_selected(Some(0));

        table_state.select_first();
        table_state.select_first_column();

        UiState {
            selected_pane: Pane::Schedule,
            schedule_table_state: table_state,
            stats_list_state,
            config_list_state,
        }
    }

    pub fn select_next_pane(&mut self) {
        match self.selected_pane {
            Pane::Schedule => self.selected_pane = Pane::AlgorithmStats,
            Pane::AlgorithmStats => self.selected_pane = Pane::Config,
            Pane::Config => self.selected_pane = Pane::Constraints,
            Pane::Constraints => self.selected_pane = Pane::Schedule,
        }
    }

    pub fn select_previous_pane(&mut self) {
        match self.selected_pane {
            Pane::Schedule => self.selected_pane = Pane::Constraints,
            Pane::AlgorithmStats => self.selected_pane = Pane::Schedule,
            Pane::Config => self.selected_pane = Pane::AlgorithmStats,
            Pane::Constraints => self.selected_pane = Pane::Config,
        }
    }
}
