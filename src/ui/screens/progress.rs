use crate::ui::{
    app::App,
    event::Key,
    layout::{centered_vertical_chunk, default_block},
};
use anyhow::Result;
use tui::{backend::Backend, Frame};

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
