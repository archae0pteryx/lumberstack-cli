use crate::ui::{
    app::{App, Selectable},
    event::Key,
};

pub fn key_handler(key: Key, app: &mut App) {
    match key {
        Key::Down => {
            app.home_menu_data.next_item();
        }
        Key::Up => {
            app.home_menu_data.prev_item();
        }
        Key::Enter => {
            let next_route = app.home_menu_data.navigate();
            app.push_route(next_route);
        }
        _ => {}
    }
}
