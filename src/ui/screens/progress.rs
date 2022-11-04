use crate::ui::{
    app::App,
    event::Key,
    layout::{default_block, default_layout},
};
use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

pub fn progress_menu() -> Vec<&'static str> {
    vec!["Back", "Quit"]
}

pub fn key_handler(key: Key, app: &mut App) {
    match key {
        Key::Down => {
            app.next_menu_item(app.home_screen_menu.len());
        }
        Key::Up => {
            app.prev_menu_item(app.home_screen_menu.len());
        }
        Key::Enter => {
            let selected = app.menu_list_state.selected();
            if let Some(s) = selected {
                match s {
                    0 => {
                        app.pop_route();
                    }
                    1 => {
                        app.quit();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn draw_progress_screen<B>(f: &mut Frame<B>, _: &App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = default_layout(layout_chunk);
    let block = default_block().title("Progress Screen");
    f.render_widget(block, chunks[0]);

    Ok(())
}
