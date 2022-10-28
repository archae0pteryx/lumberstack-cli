use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use anyhow::Result;

use crate::task_definitions::templates::tags::TaskTag;

use super::app::App;

pub fn run_tag_select(app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate =  Duration::from_millis(250);
    loop {
        // app.terminal.draw(|f| ui(f, &mut app))?;

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
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(2)
        .horizontal_margin(3)
        .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
        .split(f.size());

    // generate_list_widget(f, app, chunks[1]);
}

// fn generate_list_widget<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
//     let mut items: Vec<ListItem> = app
//         .tag_menu
//         .items
//         .iter()
//         .map(|i| {
//             ListItem::new(i.to_string()).style(Style::default())
//         })
//         .collect();

//     items.push(ListItem::new("Quit".to_string()).style(Style::default()));

//     let items = List::new(items)
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .title("Lumberstack")
//                 .border_type(BorderType::Rounded),
//         )
//         .highlight_style(
//             Style::default()
//                 .fg(Color::LightGreen)
//                 .add_modifier(Modifier::ITALIC),
//         );

//     f.render_stateful_widget(items, chunk, &mut app.tag_menu.state);
// }
