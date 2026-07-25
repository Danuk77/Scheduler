use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::ui::{Pane, components::border::create_surrounding_border};

pub fn render_tooltip(current_pane: Pane, frame: &mut Frame, area: &Rect) {
    let tooltip_string: String;
    let surrounding_border = create_surrounding_border(None);

    match current_pane {
        Pane::Schedule => {
            tooltip_string =
                String::from("Active pane: Schedule | hjkl: Move | Tab: Change pane | q: quit");
        }
        Pane::AlgorithmStats => {
            tooltip_string = String::from("Tab: Change pane");
        }

        Pane::Constraints => {
            tooltip_string = String::from("Tab: Change pane");
        }
    }

    let tooltip = Paragraph::new(tooltip_string)
        .block(surrounding_border)
        .alignment(Alignment::Center);
    frame.render_widget(tooltip, *area);
}
