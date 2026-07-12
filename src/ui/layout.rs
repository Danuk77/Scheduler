use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

/// Creates the base layout for the application
///
/// # Arguments
/// * frame: The master frame that will be split
///
/// # Returns
/// * Rc<[Rect]>: A reference counted list of rects split from the master rect
pub fn create_app_layout(frame: &mut Frame) -> [Rect; 2] {
    let [schedule_block, todo_block] = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Ratio(10, 1), Constraint::Ratio(1, 10)])
        .areas(frame.area());

    [schedule_block, todo_block]
}
