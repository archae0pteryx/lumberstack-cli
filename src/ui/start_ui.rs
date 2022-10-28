use crate::{
    app_config::AppConfig,
    lumberstack::Runnable,
    task_definitions::templates::{github::GithubTemplates, tags::TaskTag},
};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

use super::{
    app::{App, RouteId},
    key_events::handle_key_events,
    screens::{generate_all::draw_generate_all, home::draw_home},
};

// pub const BASIC_VIEW_HEIGHT: u16 = 6;
// pub const SMALL_TERMINAL_WIDTH: u16 = 150;
// pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

pub fn start_ui(app_config: Box<AppConfig>) -> Result<()> {
    setup_templates(&app_config)?;
    let mut app = App::new(app_config);

    let stdout = stdout();
    execute!(&stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let mut is_first_render = true;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        if let Ok(size) = terminal.backend().size() {
            if is_first_render || size != app.term_size {
                terminal.clear()?;
                is_first_render = false;
                app.term_size = size;
            }
        }

        terminal.draw(|f| {
            draw_main_layout(f, &app);
        })?;

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
                    _ => {
                        handle_key_events(key, &mut app);
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            terminal.show_cursor()?;
            disable_raw_mode()?;
            let mut stdout = std::io::stdout();
            execute!(stdout, LeaveAlternateScreen)?;
            terminal.clear()?;
            return Ok(());
        }
    }
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // let margin = get_main_layout_margin(app);
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

    match current_route {
        RouteId::Home => draw_home(f, app, layout_chunk),
        RouteId::GenerateAll => draw_generate_all(f, app, layout_chunk),
        _ => {}
    }
}

// pub fn get_main_layout_margin(app: &App) -> u16 {
//     if app.term_size.height > SMALL_TERMINAL_HEIGHT {
//         1
//     } else {
//         0
//     }
// }

fn setup_templates(app_config: &AppConfig) -> Result<()> {
    let res = GithubTemplates::clone_templates(TaskTag::Clone, app_config);
    if res.is_none() {
        return Ok(());
    }
    res.unwrap().run_job()?;
    Ok(())
}
