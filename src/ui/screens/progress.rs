use crate::ui::{
    app::App,
    event::Key,
    layout::{centered_vertical_chunk, default_block},
};
use anyhow::Result;
use tui::{backend::Backend, Frame};

pub fn key_handler(key: Key, _: &mut App) {
    match key {
        Key::Down => {}
        Key::Up => {}
        Key::Enter => {}
        _ => {}
    }
}

pub fn draw_progress_screen<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let layout_chunk = &app.layout_chunks;
    let chunk = centered_vertical_chunk(layout_chunk[1]);
    let block = default_block().title("Progress Screen");
    f.render_widget(block, chunk);

    Ok(())
}
