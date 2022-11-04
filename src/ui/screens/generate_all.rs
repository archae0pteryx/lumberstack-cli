use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{List, ListItem},
    Frame,
};

use crate::ui::{
    app::App,
    events::menu_key_events,
    layout::{default_block, default_layout},
};

use anyhow::Result;

pub fn draw_generate_all<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = default_layout(layout_chunk);
    let mut menu_items: Vec<(String, Box<dyn FnMut(&mut App)>)> = vec![
        (
            "Back".to_string(),
            Box::new(|app| {
                app.pop_route();
            }),
        ),
        (
            "Quit".to_string(),
            Box::new(|app| {
                app.quit();
            }),
        ),
    ];
    let listified_items = &menu_items
        .iter()
        .map(|i| ListItem::new(i.0.clone()).style(app.theme.list_item))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(default_block())
        .highlight_style(app.theme.list_highlight);

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    menu_key_events(app, &mut menu_items)?;

    Ok(())
}
