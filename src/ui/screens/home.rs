use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::ui::{app::App, ascii_tree::ascii_tree, common::default_block};

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

    let l = vec!["Generate All", "Tag Select", "Quit"];

    let menu_items = l
        .clone()
        .into_iter()
        .map(|i| ListItem::new(i).style(app.theme.list_item))
        .collect::<Vec<_>>();

    let list = List::new(menu_items)
        .block(default_block())
        .highlight_style(app.theme.list_highlight);

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    if crossterm::event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    let i = match app.list_state.selected() {
                        Some(i) => {
                            if i >= l.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    app.list_state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match app.list_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                l.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    app.list_state.select(Some(i));
                }
                KeyCode::Enter => {}
                KeyCode::Esc => {
                    app.quit();
                }
                KeyCode::Char('q') => {
                    app.quit();
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
