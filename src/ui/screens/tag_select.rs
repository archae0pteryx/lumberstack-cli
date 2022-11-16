use crate::ui::{
    app::{App, Selectable, TagListItem},
    event::Key,
    layout::{default_block, generate_basic_1_3_layout},
    theme::Theme,
};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::common::left_tree_block::render_tree_block;

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
        Key::Char('q') => {
            app.quit();
        }
        Key::Esc => {
            app.quit();
        }
        Key::Enter => {}
        _ => {}
    }
}

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

fn footer_text(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
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

fn main_list(f: &mut Frame<impl Backend>, app: &mut App, layout_chunk: Rect) {
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

fn header_text(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
    let theme = Theme::new();
    let header_text = Paragraph::new(Text::from("Available Tags"))
        .alignment(Alignment::Left)
        .block(Block::default())
        .style(theme.paragraph_style);

    f.render_widget(header_text, layout_chunk);
}

fn list_item(tag_item: &TagListItem) -> ListItem<'static> {
    if tag_item.is_selected {
        ListItem::new(format!("✓ {}", tag_item.name)).style(Style::default())
    } else {
        ListItem::new(format!("✕ {}", tag_item.name))
    }
}
