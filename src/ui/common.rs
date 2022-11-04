use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

use super::app::App;

pub fn default_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}

pub fn next_item(app: &mut App, length: usize) {
    let i = match app.list_state.selected() {
        Some(i) => {
            if i >= length - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    app.list_state.select(Some(i));
}

pub fn prev_item(app: &mut App, length: usize) {
    let i = match app.list_state.selected() {
        Some(i) => {
            if i == 0 {
                length - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    app.list_state.select(Some(i));
}
