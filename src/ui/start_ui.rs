use crate::{
    app_config::AppConfig,
    lumberstack::{Lumberstack, Runnable},
    task_definitions::templates::{github::GithubTemplates, tags::TaskTag},
};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, stdout, Stdout},
    ops::Sub,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame, Terminal,
};

use super::{
    app::{ActiveBlock, App, RouteId, Theme},
    ascii_tree::ascii_tree,
    table::{ColumnId, TableHeader, TableHeaderItem, TableId, TableItem},
    tag_select::run_tag_select,
    term,
};

pub const BASIC_VIEW_HEIGHT: u16 = 6;
pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

pub fn start_ui(app_config: Box<AppConfig>) -> Result<(), Box<dyn Error>> {
    setup_templates(&app_config);
    let mut app = App::new(app_config);

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let mut is_first_render = true;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        if let Ok(size) = terminal.backend().size() {
            if is_first_render || size != app.size {
                terminal.clear()?;
                is_first_render = false;
                app.size = size;
            }
        }
        let current_route = app.get_current_route();
        terminal.draw(|mut f| match current_route.active_block {
            _ => {
                draw_main_layout(f, &app);
            }
        });

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }

    terminal.clear();
    terminal.show_cursor()?;
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let margin = get_main_layout_margin(app);
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(13), Constraint::Max(12)].as_ref())
        .margin(2)
        .split(f.size());

    draw_routes(f, app, parent_layout[0]);
    // if app.size.width >= SMALL_TERMINAL_WIDTH {
    //     let parent_layout = Layout::default()
    //         .direction(Direction::Vertical)
    //         .constraints([Constraint::Min(1), Constraint::Length(6)].as_ref())
    //         .margin(margin)
    //         .split(f.size());
    //     draw_routes(f, app, parent_layout[0]);
    // } else {
    //     let parent_layout = Layout::default()
    //         .direction(Direction::Vertical)
    //         .constraints(
    //             [
    //                 Constraint::Length(3),
    //                 Constraint::Min(1),
    //                 Constraint::Length(6),
    //             ]
    //             .as_ref(),
    //         )
    //         .margin(margin)
    //         .split(f.size());
    //     draw_routes(f, app, parent_layout[1]);
    // }
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::Home => draw_home(f, app, layout_chunk),
        _ => {}
    }
}

fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let tree_p = generate_tree();

    f.render_widget(tree_p, chunks[0]);

    let header = TableHeader {
        id: TableId::MainMenu,
        items: vec![TableHeaderItem {
            text: "Lumberstack",
            ..Default::default()
        }],
    };

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::GenerateAll,
        current_route.hovered_block == ActiveBlock::GenerateAll,
    );

    let items = app
        .main_menu
        .iter()
        .map(|item| TableItem {
            id: item.id,
            value: item.text.clone(),
        })
        .collect::<Vec<TableItem>>();

    draw_table(
        f,
        app,
        chunks[1],
        ("Main Menu", &header),
        &items,
        app.main_menu_list_index,
        highlight_state,
    );
}

fn draw_table<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    table_layout: (&str, &TableHeader),
    items: &[TableItem],
    selected_index: usize,
    highlight_state: (bool, bool),
) where
    B: Backend,
{
    // let header = Row::new(vec!["Select", "Option"]);
    let selected_style = get_color(highlight_state, app.theme).add_modifier(Modifier::BOLD);
    let rows = items.iter().enumerate().map(|(i, item)| {
        let mut formatted_row = item.value.clone();
        let mut style = Style::default().fg(app.theme.primary); // default styling

        if Some(i) == selected_index.checked_sub(0) {
            style = selected_style;
        }

        Row::new(vec![formatted_row]).style(style)
    });

    let table = Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .widths([Constraint::Percentage(30), Constraint::Percentage(30)].as_ref());
    f.render_widget(table, layout_chunk);
}

pub fn get_color((is_active, is_hovered): (bool, bool), theme: Theme) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(theme.selected),
        (false, true) => Style::default().fg(theme.hovered),
        _ => Style::default().fg(theme.inactive),
    }
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

pub fn get_main_layout_margin(app: &App) -> u16 {
    if app.size.height > SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}

fn setup_templates(app_config: &AppConfig) {
    let res = GithubTemplates::clone_templates(TaskTag::Clone, &app_config);
    if res.is_none() {
        return;
    }
    res.unwrap().run_job();
}
