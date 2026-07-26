use ratatui::{
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

pub fn create_surrounding_border(title: Option<&str>, is_selected: bool) -> Block<'_> {
    let mut block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    if is_selected {
        block = block.style(Style::from(Color::Green));
    }

    match title {
        Some(title_text) => block.title(Span::from(title_text).blue()),
        None => block,
    }
}
