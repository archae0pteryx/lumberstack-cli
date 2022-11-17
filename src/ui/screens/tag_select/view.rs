use crate::ui::{
    app::App, layout::generate_basic_1_3_layout, screens::common::left_tree_block::render_tree_block,
};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use super::components::{header_text, main_list, footer_text};

pub fn draw_tag_select_screen<B: Backend>(f: &mut Frame<B>, app: &mut App) -> Result<()> {
    let layout_chunks = &app.layout_chunks;
    let main_container = layout_chunks[1];
    let container = generate_basic_1_3_layout().split(main_container);

    render_tree_block(f, layout_chunks[0]);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .split(container[0]);

    header_text(f, header_chunks[1]);
    main_list(f, app, container[1]);
    footer_text(f, container[2]);

    Ok(())
}
