use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{List, ListItem},
    Frame,
};

use crate::{
    ui::{
        app::{App},
        event::Key,
        layout::{default_block, default_layout},
    },
};

use anyhow::Result;

pub fn generate_screen_menu() -> Vec<&'static str> {
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

pub fn draw_generate_screen<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = default_layout(layout_chunk);

    let listified_items = app
        .generate_screen_menu
        .iter()
        .cloned()
        .map(|i| ListItem::new(i).style(app.theme.list_item))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(default_block())
        .highlight_style(app.theme.list_highlight);

    f.render_stateful_widget(list, chunks[1], &mut app.menu_list_state);

    Ok(())
}
