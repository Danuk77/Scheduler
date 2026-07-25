use ratatui::{Frame, layout::Rect};

use crate::ui::components::border::create_surrounding_border;

pub fn render_constraints(frame: &mut Frame, area: &Rect) {
    let border = create_surrounding_border(Some("Constraints"));

    frame.render_widget(border, *area);
}
