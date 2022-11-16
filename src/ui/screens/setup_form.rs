use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::{
    app::{App, FormInput},
    event::Key,
    theme::Theme,
};

use super::common::left_tree_block::render_tree_block;

pub fn key_handler(key: Key, app: &mut App) {
    match key {
        Key::Tab => {
            app.select_next_input();
        }
        Key::Char(c) => {
            app.handle_input(c);
        }
        Key::Backspace => {
            app.handle_delete();
        }
        Key::Enter => {}
        _ => {}
    }
}

pub fn draw_setup_screen<B>(f: &mut Frame<B>, app: &mut App) -> Result<()>
where
    B: Backend,
{
    let layout_chunks = &app.layout_chunks;
    render_tree_block(f, layout_chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunks[1]);

    form_block(f, app, chunks[1]);

    Ok(())
}

pub fn form_block<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Max(3),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    app.setup_screen_form_elements
        .iter()
        .enumerate()
        .for_each(|(idx, elem)| {
            f.render_widget(create_input(app, elem), chunks[idx]);
        });
}

fn create_input<'a>(app: &'a App, form_input: &'a FormInput) -> Paragraph<'a> {
    let theme = Theme::new();
    let mut block_style = theme.paragraph_style;
    if app.active_form_element == form_input.element {
        block_style = theme.input_selected;
    }
    Paragraph::new(form_input.value.clone())
        .style(theme.paragraph_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(block_style)
                .title(form_input.label.as_str()),
        )
}
