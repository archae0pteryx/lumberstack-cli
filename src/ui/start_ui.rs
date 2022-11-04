use crate::app_config::AppConfig;
use anyhow::Result;
use crossterm::{
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
    events::exit_key_events,
    screens::{
        generate_all::draw_generate_screen, home::draw_home_screen,
        tag_select::draw_tag_select_screen, progress::draw_progress_screen,
    },
};

pub fn start_ui(app_config: Box<AppConfig>) -> Result<()> {
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

        exit_key_events(&mut app)?;

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
            draw_home_screen(f, app, parent_layout[0])?;
        }
        Screen::GenerateAll => {
            draw_generate_screen(f, app, parent_layout[0])?;
        }
        Screen::TagSelect => {
            draw_tag_select_screen(f, app, parent_layout[0])?;
        }
        Screen::Progress => {
            draw_progress_screen(f, app, parent_layout[0])?;
        }
        _ => {}
    }
    Ok(())
}
