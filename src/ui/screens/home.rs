use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::{app::App, ascii_tree::ascii_tree, table::{TableItem, draw_table}};

pub fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let tree_p = generate_tree();

    f.render_widget(tree_p, chunks[0]);

    let table_items = app
        .main_menu_items
        .iter()
        .map(|item| TableItem {
            id: item.index,
            values: item.text.clone(),
        })
        .collect::<Vec<TableItem>>();

    draw_table(f, app, chunks[1], &table_items);
}

fn generate_tree() -> Paragraph<'static> {
    let tree = ascii_tree();
    Paragraph::new(tree)
        .block(generate_block())
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn generate_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}
