// use super::app::App;

// use anyhow::Result;
// use crossterm::event::{self, Event, KeyCode};

// pub fn exit_key_events(app: &mut App) -> Result<()> {
//     if crossterm::event::poll(app.get_timeout())? {
//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Esc => {
//                     app.quit();
//                 }
//                 KeyCode::Char('q') => {
//                     app.quit();
//                 }
//                 _ => {}
//             }
//         }
//     }
//     Ok(())
// }

// pub fn menu_key_events(
//     app: &mut App,
//     menu_items: &mut Vec<(String, Box<dyn FnMut(&mut App)>)>,
// ) -> Result<()> {
//     if crossterm::event::poll(app.get_timeout())? {
//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Down => {
//                     next_item(app, menu_items.len());
//                 }
//                 KeyCode::Up => {
//                     prev_item(app, menu_items.len());
//                 }
//                 KeyCode::Enter => {
//                     if let Some(i) = app.list_state.selected() {
//                         app.list_state.select(None);
//                         menu_items[i].1(app);
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }

//     Ok(())
// }
