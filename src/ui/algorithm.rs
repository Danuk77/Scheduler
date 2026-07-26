use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, ListState, Paragraph, Wrap},
};

use crate::{
    stats::OptimisationStats,
    ui::{
        Pane, UiState,
        components::{border::create_surrounding_border, list_item::create_key_value_list_item},
    },
};

pub fn render_algorithm_stats(
    frame: &mut Frame,
    area: &Rect,
    optimisation_statistics: Option<&OptimisationStats>,
    ui_state: &mut UiState,
) {
    let is_pane_selected = matches!(ui_state.selected_pane, Pane::AlgorithmStats);
    let surrounding_border = create_surrounding_border(Some("Algorithm stats"), is_pane_selected);

    if let Some(stats) = optimisation_statistics {
        _render_stats(
            stats,
            &mut ui_state.stats_list_state,
            surrounding_border,
            frame,
            area,
        );
    } else {
        let text = Paragraph::new(Line::from(
            Span::from("No statistics available. Run the optimisation algorithm to see statistics")
                .yellow(),
        ))
        .wrap(Wrap { trim: true })
        .block(surrounding_border);

        frame.render_widget(text, *area);
    }
}

fn _render_stats(
    stats: &OptimisationStats,
    list_state: &mut ListState,
    surrounding_border: Block,
    frame: &mut Frame,
    area: &Rect,
) {
    let total_number_of_iterations_ran = stats.move_count
        + stats.schedule_count
        + stats.unscheduling_scheduled_count
        + stats.unscheduling_unscheduled_count;

    let list_items = vec![
        create_key_value_list_item("Iterations", total_number_of_iterations_ran.to_string()),
        create_key_value_list_item("Resets", stats.reset_count.to_string()),
        create_key_value_list_item("Revets", stats.revert_count.to_string()),
        create_key_value_list_item("Moves", stats.move_count.to_string()),
        create_key_value_list_item("Swaps", stats.swap_count.to_string()),
        create_key_value_list_item("New schedulings", stats.schedule_count.to_string()),
        create_key_value_list_item(
            "Batch unschedulings (scheduled)",
            stats.unscheduling_scheduled_count.to_string(),
        ),
        create_key_value_list_item(
            "Batch unschedulings (unscheduled)",
            stats.unscheduling_unscheduled_count.to_string(),
        ),
    ];
    let list = List::new(list_items)
        .style(Color::White)
        .highlight_style(Style::new().magenta().bold())
        .scroll_padding(1)
        .direction(ListDirection::TopToBottom)
        .repeat_highlight_symbol(true)
        .block(surrounding_border);

    frame.render_stateful_widget(list, *area, list_state);
}
