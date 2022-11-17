use super::{
    app::App,
    screens::{
        home::draw_home_screen, new_project::draw_new_project_form,
        progress::draw_progress_screen, tag_select::view::draw_tag_select_screen, Screen,
    },
};
use anyhow::Result;
use tui::{backend::Backend, Frame};

pub fn draw_routes<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let cur_route = app.current_route();
    match cur_route {
        Screen::Home => {
            draw_home_screen(f, app)?;
        }
        Screen::NewProject => {
            draw_new_project_form(f, app)?;
        }
        Screen::TagSelect => {
            draw_tag_select_screen(f, app)?;
        }
        Screen::Progress => {
            draw_progress_screen(f, app)?;
        }
        Screen::Quit => {
            app.quit();
        }
        _ => {}
    }
    Ok(())
}
