use crate::ui::{
    app::{App, Selectable},
    event::Key,
};

pub fn key_handler(key: Key, app: &mut App) {
    match key {
        Key::Down => {
            app.tag_select_data.next_item();
        }
        Key::Up => {
            app.tag_select_data.prev_item();
        }
        Key::Right => {
            app.tag_select_data.increase_column();
        }
        Key::Left => {
            app.tag_select_data.decrease_column();
        }
        Key::Char(' ') => {
            app.tag_select_data.toggle_selected();
        }
        Key::Enter => {}
        _ => {}
    }
}
