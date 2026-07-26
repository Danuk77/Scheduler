use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
    widgets::ListItem,
};

pub fn create_key_value_list_item(item_key: &str, item_value: String) -> ListItem<'_> {
    let _formatted_list_item = Line::from(vec![
        Span::from(format!("{}: ", item_key)).yellow().italic(),
        Span::from(item_value.to_string()).green(),
    ]);

    ListItem::from(Text::from(_formatted_list_item))
}
