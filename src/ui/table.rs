use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use super::app::App;

pub struct TableItem {
    pub id: usize,
    pub values: String,
}

pub fn draw_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect, items: &[TableItem])
where
    B: Backend,
{
    // let header = Row::new(vec!["Select", "Option"]);
    let selected_style = Style::default()
        .fg(app.theme.selected)
        .add_modifier(Modifier::BOLD);
    let rows = items.iter().enumerate().map(|(i, item)| {
        let formatted_row = item.values.clone();
        let mut style = Style::default().fg(app.theme.primary);
        if app.main_menu_current_index == i {
            style = selected_style;
        }
        Row::new(vec![formatted_row]).style(style)
    });
    let table = Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .widths([Constraint::Percentage(30), Constraint::Percentage(30)].as_ref());
    f.render_widget(table, layout_chunk);
}
