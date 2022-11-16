use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::{layout::centered_vertical_chunk, theme::Theme};

use super::ascii_tree::ascii_tree;

pub fn render_tree_block(f: &mut Frame<impl Backend>, layout_chunk: Rect) {
    let centered_chunk = centered_vertical_chunk(layout_chunk);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(centered_chunk);

    let tree_p = ascii_tree_block();
    f.render_widget(tree_p, chunks[1]);
}

fn ascii_tree_block() -> Paragraph<'static> {
    let theme = Theme::new();
    let tree = ascii_tree();
    let block = Block::default()
        .border_style(theme.border_style)
        .borders(Borders::ALL)
        .border_type(theme.border_type);
        
    Paragraph::new(tree)
        .block(block)
        .style(theme.paragraph_style)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
