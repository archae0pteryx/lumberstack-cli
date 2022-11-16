use tui::{
    style::{Color, Modifier, Style},
    widgets::BorderType,
};

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    pub block_style: Style,
    pub list_item_highlight: Style,
    pub list_item_style: Style,
    pub paragraph_style: Style,
    pub input_selected: Style,
    pub border_style: Style,
    pub border_type: BorderType,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            primary_color: Color::White,
            secondary_color: Color::LightGreen,
            tertiary_color: Color::LightBlue,
            block_style: Style::default(),
            list_item_highlight: Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
            list_item_style: Style::default().fg(Color::White),
            paragraph_style: Style::default().fg(Color::White),
            input_selected: Style::default().fg(Color::Yellow),
            border_style: Style::default().fg(Color::White),
            border_type: BorderType::Rounded,
        }
    }
}
