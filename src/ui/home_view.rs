use std::{
    fmt::{self, write, Display},
    time::{Duration, Instant},
};

use crate::app_config::AppConfig;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use enum_iterator::{all, Sequence};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use super::{
    app::App, ascii_tree::ascii_tree, tag_select::run_tag_select,
    views::run_generate_view,
};

pub fn run_main_menu() -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        // app.terminal.draw(|f| draw_main_menu(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    // KeyCode::Left => app.tag_menu.unselect(),
                    // KeyCode::Down => app.tag_menu.next(),
                    // KeyCode::Up => app.tag_menu.previous(),
                    // KeyCode::Enter => app.tag_menu.handle_selection(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

pub fn draw_home<B: Backend>(f: &mut Frame<B>) {
    let content_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tree_p = generate_tree();

    f.render_widget(tree_p, content_container[0]);

}

fn generate_tree() -> Paragraph<'static> {
    let tree = ascii_tree();
    Paragraph::new(tree)
        .block(generate_block())
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn generate_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}

// // fn generate_list_widget<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
// //     let mut items: Vec<ListItem> = app
// //         .main_menu
// //         .items
// //         .iter()
// //         .map(|i| ListItem::new(i.to_string()).style(Style::default()))
// //         .collect();

// //     items.push(ListItem::new("Quit".to_string()).style(Style::default()));

// //     let items = List::new(items)
// //         .block(
// //             Block::default()
// //                 .borders(Borders::ALL)
// //                 .title("Lumberstack")
// //                 .border_type(BorderType::Rounded),
// //         )
// //         .highlight_style(
// //             Style::default()
// //                 .fg(Color::LightGreen)
// //                 .add_modifier(Modifier::ITALIC),
// //         );

// //     f.render_stateful_widget(items, chunk, &mut app.tag_menu.state);
// // }
