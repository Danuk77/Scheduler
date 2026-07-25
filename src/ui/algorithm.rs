use ratatui::{Frame, layout::Rect};

use crate::ui::components::border::create_surrounding_border;

pub fn render_algorithm_stats(frame: &mut Frame, area: &Rect) {
    let border = create_surrounding_border(Some("Algorithm"));

    frame.render_widget(border, *area);
}
