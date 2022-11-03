use crate::{
    app_config::AppConfig,
    lumberstack::Runnable,
    task_definitions::templates::{github::GithubTemplates, tags::TaskTag},
};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

use super::{
    app::{App, Screen},
    screens::home::draw_home,
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

    loop {
        if let Ok(size) = terminal.backend().size() {
            if is_first_render || size != app.term_size {
                terminal.clear()?;
                is_first_render = false;
                app.term_size = size;
            }
        }

        terminal.draw(|f| {
            draw_main_layout(f, &mut app).unwrap();
        })?;

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

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(13), Constraint::Max(12)].as_ref())
        .margin(2)
        .split(f.size());

    draw_routes(f, app, parent_layout[0])?;
    Ok(())
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let current_route = app.current_route();

    match current_route {
        Screen::Home => {
            draw_home(f, app, layout_chunk)?;
        }
        // Screen::GenerateAll => draw_generate_all(f, app, layout_chunk),
        _ => {}
    }
    Ok(())
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
