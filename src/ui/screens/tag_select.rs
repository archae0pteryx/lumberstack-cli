use crate::ui::{
    app::App,
    layout::{default_block, default_layout}, event::Key,
};
use anyhow::Result;
use tui::{backend::Backend, layout::Rect, Frame};

pub fn tag_select_menu() -> Vec<&'static str> {
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
            match selected {
                Some(s) => match s {
                    0 => {
                        app.pop_route();
                    }
                    1 => {
                        app.quit();
                    }
                    _ => {}
                },
                None => {}
            }
        }
        _ => {}
    }
}
pub fn draw_tag_select_screen<B>(f: &mut Frame<B>, _: &App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = default_layout(layout_chunk);
    let block = default_block().title("Tag Select");
    f.render_widget(block, chunks[0]);

    Ok(())
}
