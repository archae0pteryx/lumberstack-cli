use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::BorderType,
};

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    pub success_color: Color,
    pub error_color: Color,
    pub warn_color: Color,
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
            success_color: Color::LightGreen,
            warn_color: Color::LightYellow,
            error_color: Color::LightRed,
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

#[derive(Debug, Clone)]
pub struct ThemedText {
    pub theme: Theme,
}

impl ThemedText {
    pub fn new() -> Self {
        let theme = Theme::new();
        ThemedText { theme }
    }

    pub fn error(&self, text: &str) -> Span<'static> {
        Span::styled(
            text.to_string(),
            Style::default().fg(self.theme.error_color),
        )
    }

    pub fn success(&self, text: &str) -> Span<'static> {
        Span::styled(
            text.to_string(),
            Style::default().fg(self.theme.success_color),
        )
    }

    pub fn light_red(&self, text: &str) -> Span<'static> {
        Span::styled(text.to_string(), Style::default().fg(Color::LightRed))
    }

    pub fn light_green(&self, text: &str) -> Span<'static> {
        Span::styled(text.to_string(), Style::default().fg(Color::LightGreen))
    }
}
