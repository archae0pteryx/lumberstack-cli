use crate::ui::{
    app::App,
    layout::default_block,
    theme::{Theme, ThemedText},
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::tag_menu::TagListItem;

pub fn main_list(f: &mut Frame<impl Backend>, app: &mut App, layout_chunk: Rect) {
    let theme = Theme::new();
    let tag_columns = app.tag_select_data.tag_columns.clone();
    let num_cols = tag_columns.len();
    let constraints = vec![Constraint::Max(20); num_cols + 1];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(layout_chunk);

    tag_columns.into_iter().enumerate().for_each(|(i, tc)| {
        let items = tc.items.iter().map(list_item).collect::<Vec<_>>();
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(theme.list_item_highlight);
        f.render_stateful_widget(items, chunks[i], &mut tc.state.clone());
    });
}

pub fn header_text(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
    let theme = Theme::new();
    let header_text = Paragraph::new(Text::from("Available Modules"))
        .alignment(Alignment::Left)
        .block(Block::default())
        .style(theme.paragraph_style);

    f.render_widget(header_text, layout_chunk);
}

pub fn footer_text(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
    let footer_text = vec![
        Spans::from(Span::styled(
            "Press space to select/deselect",
            Style::default().fg(Color::LightYellow),
        )),
        Spans::from(Span::styled(
            "Press <enter> to run selected.",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let footer_paragraph = Paragraph::new(footer_text)
        .block(default_block().borders(Borders::NONE))
        .alignment(Alignment::Left);

    f.render_widget(footer_paragraph, layout_chunk);
}

fn list_item(tag_item: &TagListItem) -> ListItem<'static> {
    let text = ThemedText::new();

    let tag_name_deselected = text.light_red(&tag_item.name.to_string());
    let tag_name_selected = text.light_green(&tag_item.name.to_string());

    if tag_item.is_selected {
        ListItem::new(Spans::from(vec![text.success("✓ "), tag_name_selected]))
    } else {
        ListItem::new(Spans::from(vec![text.error("✕ "), tag_name_deselected]))
    }
}
