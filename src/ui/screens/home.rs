use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::ui::{
    app::{App, Screen},
    ascii_tree::ascii_tree,
    event::Key,
    layout::default_block,
};

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

pub fn draw_home_screen<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let tree_p = ascii_tree_block();
    f.render_widget(tree_p, chunks[0]);

    let listified_items = app
        .home_screen_menu
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

fn ascii_tree_block() -> Paragraph<'static> {
    let tree = ascii_tree();
    Paragraph::new(tree)
        .block(tree_block())
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn tree_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}
