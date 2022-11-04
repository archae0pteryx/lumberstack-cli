#![allow(unused)]
use tui::{backend::Backend, layout::{Rect, Layout, Direction, Constraint}, Frame};

use crate::ui::{app::App, common::default_block};

pub fn draw_tag_select<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let block = default_block().title("Generate All");

    f.render_widget(block, chunks[0]);
}
