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
use std::io::stdout;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use super::{
    app::{App, Screen},
    screens::{generate_all::draw_generate_all, home::draw_home, tag_select::draw_tag_select},
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

    loop {
        if let Ok(size) = terminal.backend().size() {
            if app.is_first_render || size != app.term_size {
                terminal.clear()?;
                app.is_first_render = false;
                app.term_size = size;
            }
        }

        terminal.draw(|f| {
            draw_routes(f, &mut app).unwrap();
        })?;

        global_key_events(&mut app)?;

        if app.should_quit {
            terminal.show_cursor()?;
            disable_raw_mode()?;
            let mut stdout = std::io::stdout();
            execute!(stdout, LeaveAlternateScreen)?;
            terminal.clear()?;
            return Ok(());
        }

        app.tick();
    }
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(13), Constraint::Max(12)].as_ref())
        .margin(2)
        .split(f.size());

    let current_route = app.current_route();

    match current_route {
        Screen::Home => {
            draw_home(f, app, parent_layout[0])?;
        }
        Screen::GenerateAll => draw_generate_all(f, app, parent_layout[0]),
        Screen::TagSelect => draw_tag_select(f, app, parent_layout[0]),
        _ => {}
    }
    Ok(())
}

fn global_key_events(app: &mut App) -> Result<()> {
    if crossterm::event::poll(app.get_timeout())? {
        if let Event::Key(key) = event::read()? {
            match key.code {
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
