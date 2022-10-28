use crossterm::event::{KeyCode, KeyEvent};

use super::app::App;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    // let current_route = app.get_current_route();
    // match current_route {
    //     _ => {
    //         common_event_keys(key, app);
    //     }
    // }
    common_event_keys(key, app);
}

fn common_event_keys(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Down => {
            app.next_item();
        }
        KeyCode::Up => {
            app.prev_item();
        }
        KeyCode::Enter => {
            app.handle_submit();
        }
        _ => {}
    }
}
