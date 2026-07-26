use std::vec;

use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table},
};

use crate::{
    constraints::constraint_store::ConstraintStore,
    schedule::Schedule,
    ui::{Pane, UiState, components::border::create_surrounding_border},
};

pub fn render_schedule(
    schedule: &Schedule,
    frame: &mut Frame,
    area: &Rect,
    constraint_store: &ConstraintStore,
    ui_state: &mut UiState,
) {
    let is_pane_selected = matches!(ui_state.selected_pane, Pane::Schedule);
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
        .block(create_surrounding_border(
            Some("Schedule"),
            is_pane_selected,
        ))
        .cell_highlight_style(Style::new().reversed().yellow());

    frame.render_stateful_widget(table, *area, &mut ui_state.schedule_table_state);
}

fn _generate_table_rows(
    grid: &[[Option<u32>; 48]; 7],
    constraint_store: &ConstraintStore,
) -> Vec<Row<'static>> {
    let step_size = 30;

    (0..48)
        .map(|i| {
            let current_minutes = i * step_size;
            let scheduled_constraints: Vec<Cell> = (0..7)
                .map(|j| _create_cell(grid[j][i], constraint_store))
                .collect();

            let mut cells: Vec<Cell> = vec![Cell::new(format!(
                "{:02}:{:02}",
                current_minutes / 60,
                current_minutes % 60,
            ))];
            cells.extend(scheduled_constraints);

            Row::new(cells)
        })
        .collect()
}

/// Creates a cell for the specified constraint
///
/// # Args:
/// * `constraint_id` (option<u32>): The id of the constraint or None if it is a free cell
/// * `constraint_store` (&ConstraintStore): The constraint store
///
/// # Returns
/// Cell: A generated cell ready to be rendered by the UI
fn _create_cell(constraint_id: Option<u32>, constraint_store: &ConstraintStore) -> Cell<'static> {
    let constraint_name: String = constraint_id.map_or("free".to_string(), |id| {
        constraint_store
            .get_constraint_name(id)
            .unwrap_or("error".to_string())
    });
    let cell_color: Color = constraint_id.map_or(Color::White, |id| _get_color_for_constraint(id));

    Cell::new(constraint_name).style(Style::new().fg(cell_color))
}

// TODO: Maybe base it on constraint type instead of id
fn _get_color_for_constraint(constraint_id: u32) -> Color {
    const PALETTE: [Color; 16] = [
        Color::Rgb(138, 173, 244), // soft blue
        Color::Rgb(166, 218, 149), // sage green
        Color::Rgb(245, 169, 127), // peach
        Color::Rgb(198, 160, 246), // lavender
        Color::Rgb(139, 213, 202), // teal
        Color::Rgb(238, 153, 160), // rose
        Color::Rgb(238, 212, 159), // sand
        Color::Rgb(125, 196, 228), // sky
        Color::Rgb(183, 189, 130), // olive
        Color::Rgb(244, 154, 194), // pink
        Color::Rgb(145, 215, 227), // ice
        Color::Rgb(210, 178, 140), // tan
        Color::Rgb(149, 205, 165), // mint
        Color::Rgb(216, 160, 223), // orchid
        Color::Rgb(240, 198, 116), // gold
        Color::Rgb(160, 170, 215), // periwinkle
    ];
    PALETTE[constraint_id as usize % PALETTE.len()]
}
