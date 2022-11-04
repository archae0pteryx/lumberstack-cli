#![allow(unused)]
use tui::{backend::Backend, layout::{Rect, Layout, Direction, Constraint}, Frame};

use crate::ui::{app::App, common::{default_block, default_layout}, events::menu_key_events};

pub fn draw_tag_select<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = default_layout(layout_chunk);
    let block = default_block().title("Tag Select");
    f.render_widget(block, chunks[0]);
}
