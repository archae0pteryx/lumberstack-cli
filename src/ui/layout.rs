use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

// pub const BASIC_VIEW_HEIGHT: u16 = 6;
// pub const SMALL_TERMINAL_WIDTH: u16 = 150;
// pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

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
// pub fn get_main_layout_margin(app: &App) -> u16 {
//     if app.term_size.height > SMALL_TERMINAL_HEIGHT {
//         1
//     } else {
//         0
//     }
// }
