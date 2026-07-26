use ratatui::{
    style::Stylize,
    text::Span,
    widgets::{Block, BorderType, Borders},
};

pub fn create_surrounding_border(title: Option<&str>) -> Block<'_> {
    let block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    match title {
        Some(title_text) => block.title(Span::from(title_text).blue()),
        None => block,
    }
}
