use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{List, ListDirection, ListItem, ListState},
};

use crate::{stats::OptimisationStats, ui::components::border::create_surrounding_border};

pub fn render_algorithm_stats(
    frame: &mut Frame,
    area: &Rect,
    optimisation_statistics: Option<&OptimisationStats>,
    throughput_stats_list_state: &mut ListState,
) {
    if let Some(stats) = optimisation_statistics {
        _render_stats(stats, throughput_stats_list_state, frame, area);
    }
}

fn _render_stats(
    stats: &OptimisationStats,
    list_state: &mut ListState,
    frame: &mut Frame,
    area: &Rect,
) {
    let surrounding_border = create_surrounding_border(Some("Algorithm stats"));
    let total_number_of_iterations_ran = stats.move_count
        + stats.schedule_count
        + stats.unscheduling_scheduled_count
        + stats.unscheduling_unscheduled_count;

    let list_items = vec![
        _format_stat("Iterations", total_number_of_iterations_ran),
        _format_stat("Resets", stats.reset_count),
        _format_stat("Revets", stats.revert_count),
        _format_stat("Moves", stats.move_count),
        _format_stat("Swaps", stats.swap_count),
        _format_stat("New schedulings", stats.schedule_count),
        // TODO: Rename
        _format_stat(
            "Batch unschedulings (scheduled)",
            stats.unscheduling_scheduled_count,
        ),
        // TODO: Rename
        _format_stat(
            "Batch unschedulings (unscheduled)",
            stats.unscheduling_unscheduled_count,
        ),
    ];
    let list = List::new(list_items)
        .style(Color::White)
        .highlight_style(Style::new().red())
        .scroll_padding(1)
        .direction(ListDirection::TopToBottom)
        .repeat_highlight_symbol(true)
        .block(surrounding_border);

    frame.render_widget(list, *area);
}

fn _format_stat(stat_name: &str, stat_value: u32) -> ListItem<'_> {
    let formatted_stat = Line::from(vec![
        Span::from(format!("{}: ", stat_name)).yellow().italic(),
        Span::from(stat_value.to_string()).green(),
    ]);

    ListItem::from(Text::from(formatted_stat))
}
