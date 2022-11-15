use super::{
    app::{App, Screen},
    screens::{
        generate_all::draw_generate_screen, home::draw_home_screen, progress::draw_progress_screen,
        setup_form::draw_setup_screen, tag_select::draw_tag_select_screen,
    },
};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

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
        Screen::Setup => {
            draw_setup_screen(f, app, parent_layout[0])?;
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
