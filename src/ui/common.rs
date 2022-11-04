use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

pub fn default_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}

pub fn default_layout(layout_chunk: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(layout_chunk)
}
