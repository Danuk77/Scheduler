use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

pub struct ScheduleScreenBlocks {
    pub schedule_block: Rect,
    pub algorithm_stats_block: Rect,
    pub config: Rect,
    pub constraints_block: Rect,
    pub tooltip_block: Rect,
}

/// Creates the base layout for the application
///
/// # Arguments
/// * frame: The master frame that will be split
///
/// # Returns
/// TODO: Complete
pub fn create_app_layout(frame: &mut Frame) -> ScheduleScreenBlocks {
    let [_content_block, tooltip_block] = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .areas(frame.area());

    let [schedule_block, _controls_block] = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(40)])
        .areas(_content_block);

    let [algorithm_stats_block, config, constraints_block] = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .areas(_controls_block);

    ScheduleScreenBlocks {
        schedule_block,
        algorithm_stats_block,
        config,
        constraints_block,
        tooltip_block,
    }
}
