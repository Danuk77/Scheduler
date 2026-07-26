use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::ui::{Pane, components::border::create_surrounding_border};

pub fn render_tooltip(selected_pane: Pane, frame: &mut Frame, area: &Rect) {
    let tooltip_string: String;
    let surrounding_border = create_surrounding_border(None, false);

    match selected_pane {
        Pane::Schedule => {
            tooltip_string = String::from(
                "Active pane: Schedule | hjkl: Move | Tab: Select next pane | Shif+Tab: Select previous pane | q: quit",
            );
        }
        Pane::AlgorithmStats => {
            tooltip_string = String::from(
                "Active pane: Algorithm stats | Tab: Change pane | Shift+Tab: Select previous pane | q: quit",
            );
        }

        Pane::Constraints => {
            tooltip_string = String::from(
                "Active pane: Constraints | Tab: Change pane | Shift+Tab: Select previous pane | q: quit",
            );
        }

        Pane::Config => {
            tooltip_string = String::from(
                "Active pane: Config | Tab: Change pane | Shift+Tab: Select previous pane | q: quit",
            );
        }
    }

    let tooltip = Paragraph::new(tooltip_string)
        .block(surrounding_border)
        .alignment(Alignment::Center);
    frame.render_widget(tooltip, *area);
}
