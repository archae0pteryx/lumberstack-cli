#![allow(unused)]
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::ui::{
    app::App,
    common::{default_block, next_item, prev_item},
};

pub fn draw_generate_all<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    let menu_items: Vec<(String, Box<dyn FnMut(&mut App)>)> = vec![(
        "Quit".to_string(),
        Box::new(|app| {
            app.quit();
        }),
    )];
    let listified_items = &menu_items
        .iter()
        .map(|i| ListItem::new(i.0.clone()).style(app.theme.list_item))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(default_block())
        .highlight_style(app.theme.list_highlight);

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);
}

fn handle_keypress(app: &mut App, length: usize) -> Result<()> {
    if crossterm::event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    next_item(app, length);
                }
                KeyCode::Up => {
                    prev_item(app, length);
                }
                KeyCode::Enter => {
                    // if let Some(i) = app.list_state.selected() {
                    //     menu_items[i].1(app);
                    // }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
