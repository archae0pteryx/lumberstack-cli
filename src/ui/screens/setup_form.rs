use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::{
    app::{App, FormInput},
    ascii_tree::ascii_tree,
    event::Key,
};

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

pub fn draw_setup_screen<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) -> Result<()>
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(13), Constraint::Length(93)].as_ref())
        .split(layout_chunk);

    let tree_p = ascii_tree_block();
    f.render_widget(tree_p, chunks[0]);

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
    let mut block_style = app.theme.paragraph_style;
    if app.active_form_element == form_input.element {
        block_style = app.theme.input_selected;
    }
    Paragraph::new(form_input.value.clone())
        .style(app.theme.paragraph_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(block_style)
                .title(form_input.label.as_str()),
        )
}

fn ascii_tree_block() -> Paragraph<'static> {
    let tree = ascii_tree();
    Paragraph::new(tree)
        .block(tree_block())
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn tree_block() -> Block<'static> {
    let border_style = Style::default().fg(Color::LightGreen);
    let border_type = BorderType::Rounded;
    Block::default()
        .style(Style::default())
        .border_style(border_style)
        .borders(Borders::ALL)
        .border_type(border_type)
}
