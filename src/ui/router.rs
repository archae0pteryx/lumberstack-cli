use super::{
    app::{App, Screen},
    screens::{
        generate_all::draw_generate_screen, home::draw_home_screen, progress::draw_progress_screen,
        setup_form::draw_setup_screen, tag_select::draw_tag_select_screen,
    },
};
use anyhow::Result;
use tui::{backend::Backend, Frame};

pub fn draw_routes<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let current_route = app.current_route();
    match current_route {
        Screen::Home => {
            draw_home_screen(f, app)?;
        }
        Screen::Setup => {
            draw_setup_screen(f, app)?;
        }
        Screen::GenerateAll => {
            draw_generate_screen(f, app)?;
        }
        Screen::TagSelect => {
            draw_tag_select_screen(f, app)?;
        }
        Screen::Progress => {
            draw_progress_screen(f, app)?;
        }
        _ => {}
    }
    Ok(())
}
