use anyhow::Result;
use tui::{
    backend::Backend,
    layout::Alignment,
    widgets::{List, ListItem},
    Frame,
};

use crate::ui::{
    app::{App, Screen},
    event::Key,
    layout::{centered_vertical_chunk, default_block},
    theme::Theme,
};

use super::common::left_tree_block::render_tree_block;

pub fn home_screen_menu() -> Vec<&'static str> {
    vec!["New Project", "Generate All", "Select Tags", "Quit"]
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
                        app.push_route(Screen::Setup);
                    }
                    1 => {
                        app.push_route(Screen::GenerateAll);
                    }
                    2 => {
                        app.push_route(Screen::TagSelect);
                    }
                    3 => {
                        app.quit();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn draw_home_screen<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let theme = Theme::new();
    let main_layout_chunks = &app.layout_chunks;
    render_tree_block(f, main_layout_chunks[0]);

    let right_layout_chunk = centered_vertical_chunk(main_layout_chunks[1]);

    let listified_items = app
        .home_screen_menu
        .iter()
        .cloned()
        .map(|i| ListItem::new(i).style(theme.list_item_style))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(
            default_block()
                .title("Lumberstack")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(theme.list_item_highlight);

    f.render_stateful_widget(list, right_layout_chunk, &mut app.menu_list_state);

    Ok(())
}
