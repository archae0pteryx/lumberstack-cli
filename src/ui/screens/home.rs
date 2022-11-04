use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
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
    common::{default_block, next_item, prev_item},
};

pub fn draw_home<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let tree_p = ascii_tree_block();
    f.render_widget(tree_p, chunks[0]);

    let mut menu_items: Vec<(String, Box<dyn FnMut(&mut App)>)> = vec![
        (
            "Generate All".to_string(),
            Box::new(|app| {
                app.push_route(Screen::GenerateAll);
            }),
        ),
        (
            "Select Tags".to_string(),
            Box::new(|app| {
                app.push_route(Screen::TagSelect);
            }),
        ),
        (
            "Quit".to_string(),
            Box::new(|app| {
                app.quit();
            }),
        ),
    ];

    let listified_items = &menu_items
        .iter()
        .map(|i| ListItem::new(i.0.clone()).style(app.theme.list_item))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(default_block())
        .highlight_style(app.theme.list_highlight);

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    home_key_events(app, &mut menu_items)?;

    Ok(())
}

fn home_key_events(
    app: &mut App,
    menu_items: &mut Vec<(String, Box<dyn FnMut(&mut App)>)>,
) -> Result<()> {
    if crossterm::event::poll(app.get_timeout())? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    next_item(app, menu_items.len());
                }
                KeyCode::Up => {
                    prev_item(app, menu_items.len());
                }
                KeyCode::Enter => {
                    if let Some(i) = app.list_state.selected() {
                        app.list_state.select(None);
                        menu_items[i].1(app);
                    }
                }
                _ => {}
            }
        }
    }

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
