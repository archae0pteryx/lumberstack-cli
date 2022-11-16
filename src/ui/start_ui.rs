use crate::app_config::AppConfig;
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Stdout};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};

use super::{
    app::App,
    event::{self, Key},
    handlers,
    router::draw_routes,
};

pub fn start_ui(app_config: AppConfig) -> Result<()> {
    let mut app = App::new(app_config);

    let stdout = stdout();
    execute!(&stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    let events = event::Events::new(250);
    loop {
        if app.should_quit {
            break;
        }

        if app.is_first_render {
            terminal.clear()?;
            app.is_first_render = false;
            app.update_on_tick(terminal.size().unwrap());
        }

        terminal.draw(|f| {
            draw_routes(f, &mut app).unwrap();
        })?;

        match events.next()? {
            event::Event::Input(key) => match key {
                Key::Ctrl('c') => {
                    app.quit();
                }
                Key::Char('q') => {
                    app.quit();
                }
                Key::Esc => {
                    app.quit();
                }
                _ => {
                    handlers::handle_app(key, &mut app);
                }
            },
            event::Event::Tick => {
                app.update_on_tick(terminal.size().unwrap());
            }
        }
    }
    quit_terminal(&mut terminal)?;
    Ok(())
}

fn quit_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal.show_cursor()?;
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    terminal.clear()?;
    Ok(())
}
