use super::{app::App, event::Key, screens::{self, Screen}};

pub fn handle_app(key: Key, app: &mut App) {
    let current_route = app.current_route();
    match current_route {
        Screen::Home => {
            screens::home::controls::key_handler(key, app);
        }
        Screen::NewProject => {
            screens::new_project::key_handler(key, app);
        }
        Screen::TagSelect => {
            screens::tag_select::controls::key_handler(key, app);
        }
        Screen::Progress => {
            screens::progress::key_handler(key, app);
        }
        _ => {}
    }
}
