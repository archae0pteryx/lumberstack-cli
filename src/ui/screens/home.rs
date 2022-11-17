use anyhow::Result;
use tui::{
    backend::Backend,
    layout::Alignment,
    widgets::{List, ListItem},
    Frame,
};

use crate::ui::{
    app::App,
    layout::{centered_vertical_chunk, default_block},
    theme::Theme,
};

use super::common::left_tree_block::render_tree_block;

pub(crate) mod controls;
pub(crate) mod home_menu;

pub fn draw_home_screen<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let theme = Theme::new();
    let main_layout_chunks = &app.layout_chunks;
    render_tree_block(f, main_layout_chunks[0]);

    let right_layout_chunk = centered_vertical_chunk(main_layout_chunks[1]);

    let listified_items = app
        .home_menu_data
        .items
        .iter()
        .cloned()
        .map(|i| ListItem::new(i.0).style(theme.list_item_style))
        .collect::<Vec<_>>();

    let list = List::new(listified_items.clone())
        .block(
            default_block()
                .title("Lumberstack")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(theme.list_item_highlight);

    f.render_stateful_widget(list, right_layout_chunk, &mut app.home_menu_data.state);

    Ok(())
}
