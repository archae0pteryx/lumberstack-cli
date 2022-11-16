use tui::{
    backend::Backend,
    widgets::{List, ListItem},
    Frame,
};

use crate::ui::{
    app::App,
    event::Key,
    layout::{default_block, centered_vertical_chunk}, theme::Theme,
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

pub fn draw_generate_screen<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let theme = Theme::new();
    let layout_chunk = &app.layout_chunks;
    let chunks = centered_vertical_chunk(layout_chunk[1]);

    let listified_items = app
        .generate_screen_menu
        .iter()
        .cloned()
        .map(|i| ListItem::new(i).style(theme.list_item_style))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(default_block())
        .highlight_style(theme.list_item_highlight);

    f.render_stateful_widget(list, chunks, &mut app.menu_list_state);

    Ok(())
}
