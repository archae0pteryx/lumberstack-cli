use crate::ui::{
    app::App,
    layout::{default_block, default_layout},
};
use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

pub fn draw_progress_screen<B>(f: &mut Frame<B>, _: &App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = default_layout(layout_chunk);
    let block = default_block().title("Progress Screen");
    f.render_widget(block, chunks[0]);

    Ok(())
}
