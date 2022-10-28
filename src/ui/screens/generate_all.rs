use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::ui::app::App;

pub fn draw_generate_all<B>(f: &mut Frame<B>, _: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let block = generate_block();

    f.render_widget(block, chunks[0]);
}
fn generate_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}
