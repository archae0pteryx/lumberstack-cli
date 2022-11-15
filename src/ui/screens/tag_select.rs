use crate::ui::{
    app::{App, Selectable, TagListItem},
    event::Key,
    layout::default_block,
};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
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

pub fn draw_tag_select_screen<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    layout_chunk: Rect,
) -> Result<()> {
    let screen_container = screen_container(layout_chunk);

    header_text(f, screen_container[0]);
    main_list(f, app, screen_container[1]);
    footer_text(f, screen_container[2]);

    Ok(())
}

fn footer_text(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
    let footer_text = vec![
        Spans::from(Span::styled(
            "Press space to select/deselect a tag. Press enter to continue.",
            Style::default().fg(Color::LightGreen),
        )),
        Spans::from(Span::styled(
            "Press q to quit.",
            Style::default().fg(Color::LightGreen),
        )),
    ];

    let footer_paragraph = Paragraph::new(footer_text)
        .block(default_block().borders(Borders::NONE))
        .alignment(tui::layout::Alignment::Center);

    f.render_widget(footer_paragraph, layout_chunk);
}

fn main_list(f: &mut Frame<impl Backend>, app: &mut App, layout_chunk: Rect) {
    let tag_columns = app.tag_select_data.tag_columns.clone();
    let num_cols = tag_columns.len();
    let constraints = vec![Constraint::Percentage(100 / num_cols as u16); num_cols];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(layout_chunk);

    tag_columns.into_iter().enumerate().for_each(|(i, tc)| {
        let items = tc.items.iter().map(list_item).collect::<Vec<_>>();
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(items, chunks[i], &mut tc.state.clone());
    });
}

fn header_text(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
    let header_text = Paragraph::new(Text::from("Available Tags"))
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::Green));

    f.render_widget(header_text, layout_chunk);
}

fn screen_container(layout_chunk: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(layout_chunk)
}

fn list_item(tag_item: &TagListItem) -> ListItem<'static> {
    if tag_item.is_selected {
        ListItem::new(format!("✓ {}", tag_item.name)).style(Style::default())
    } else {
        ListItem::new(format!("✕ {}", tag_item.name))
    }
}
