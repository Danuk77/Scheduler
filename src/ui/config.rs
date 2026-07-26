use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{List, ListDirection},
};

use crate::{
    config::Config,
    ui::{
        Pane, UiState,
        components::{border::create_surrounding_border, list_item::create_key_value_list_item},
    },
};

pub fn render_config(config: &Config, frame: &mut Frame, area: &Rect, ui_state: &mut UiState) {
    let is_pane_selected = matches!(ui_state.selected_pane, Pane::Config);
    let surrounding_border = create_surrounding_border(Some("Config"), is_pane_selected);
    let list_items = vec![
        create_key_value_list_item("iterations", config.iterations.to_string()),
        create_key_value_list_item(
            "number of global searches",
            config.number_of_global_searches.to_string(),
        ),
        create_key_value_list_item(
            "initial temperature",
            config.initial_temperature.to_string(),
        ),
        create_key_value_list_item("cooling factor", config.cooling_factor.to_string()),
        create_key_value_list_item("random seed", config.random_seed.to_string()),
        create_key_value_list_item(
            "presence high penalty",
            config.penalties_config.presence_high.to_string(),
        ),
        create_key_value_list_item(
            "presence low penalty",
            config.penalties_config.presence_low.to_string(),
        ),
        create_key_value_list_item(
            "allowed slots high penalty",
            config.penalties_config.allowed_slots_high.to_string(),
        ),
        create_key_value_list_item(
            "allowed slots low penalty",
            config.penalties_config.allowed_slots_low.to_string(),
        ),
        create_key_value_list_item(
            "preferred slots high penalty",
            config.penalties_config.preferred_slots_high.to_string(),
        ),
        create_key_value_list_item(
            "preferred slots low penalty",
            config.penalties_config.preferred_slots_low.to_string(),
        ),
        create_key_value_list_item(
            "gap high penalty",
            config.penalties_config.gap_high.to_string(),
        ),
        create_key_value_list_item(
            "gap low penalty",
            config.penalties_config.gap_low.to_string(),
        ),
        create_key_value_list_item(
            "move chance (optimisation strategy)",
            config.optimisation_strategy_config.move_chance.to_string(),
        ),
        create_key_value_list_item(
            "unschedule chance (optimisation strategy)",
            config
                .optimisation_strategy_config
                .unschedule_chance
                .to_string(),
        ),
        create_key_value_list_item(
            "swap chance (optimisation strategy)",
            config.optimisation_strategy_config.swap_chance.to_string(),
        ),
    ];

    let list = List::new(list_items)
        .style(Color::White)
        .highlight_style(Style::new().magenta().bold())
        .scroll_padding(1)
        .direction(ListDirection::TopToBottom)
        .repeat_highlight_symbol(true)
        .block(surrounding_border);

    frame.render_stateful_widget(list, *area, &mut ui_state.config_list_state);
}
