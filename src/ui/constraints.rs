use ratatui::{Frame, layout::Rect};

use crate::ui::{Pane, UiState, components::border::create_surrounding_border};

pub fn render_constraints(frame: &mut Frame, area: &Rect, ui_state: &mut UiState) {
    let is_pane_selected = matches!(ui_state.selected_pane, Pane::Constraints);
    let border = create_surrounding_border(Some("Constraints"), is_pane_selected);

    frame.render_widget(border, *area);
}
