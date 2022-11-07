use super::{
    app::{App, Screen},
    event::Key,
    screens,
};

pub fn handle_app(key: Key, app: &mut App) {
    let current_route = app.current_route();
    match current_route {
        Screen::Home => {
            screens::home::key_handler(key, app);
        }
        Screen::Setup => {
            screens::setup_form::key_handler(key, app);
        }
        Screen::GenerateAll => {
            screens::generate_all::key_handler(key, app);
        }
        Screen::TagSelect => {
            screens::tag_select::key_handler(key, app);
        }
        Screen::Progress => {
            screens::progress::key_handler(key, app);
        }
        _ => {}
    }
}
