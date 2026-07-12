use std::vec;

use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Row, Table, TableState},
};

use crate::{constraints::constraint_store::ConstraintStore, schedule::Schedule};

pub fn render_schedule(
    schedule: &Schedule,
    frame: &mut Frame,
    area: &Rect,
    table_state: &mut TableState,
    constraint_store: &ConstraintStore,
) {
    let header = Row::new(vec![
        "Time", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
    ]);
    let widths = [
        Constraint::Percentage(9),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
    ];

    let rows = _generate_table_rows(&schedule.grid, constraint_store);

    let table = Table::new(rows, widths)
        .header(header.style(Style::new().bold()))
        .block(Block::bordered().title("Schedule"))
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow());

    frame.render_stateful_widget(table, *area, table_state);
}

fn _generate_table_rows(
    grid: &[[Option<u32>; 48]; 7],
    constraint_store: &ConstraintStore,
) -> Vec<Row<'static>> {
    let step_size = 30;

    (0..48)
        .map(|i| {
            let current_minutes = i * step_size;
            let scheduled_constraints: Vec<String> = (0..7)
                .map(|j| {
                    grid[j][i].map_or("FREE".to_string(), |constraint_id| {
                        constraint_store
                            .get_constraint_name(constraint_id)
                            .unwrap_or("error".to_string())
                    })
                })
                .collect();
            let mut cells = vec![format!(
                "{:02}:{:02}",
                current_minutes / 60,
                current_minutes % 60
            )];
            cells.extend(scheduled_constraints);

            Row::new(cells)
        })
        .collect()
}
