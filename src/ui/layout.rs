#![allow(unused)]
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{
    widgets::{Block, Borders},
};

use super::theme::Theme;

pub static LARGE_LAYOUT: Rect = Rect {
    x: 0,
    y: 0,
    width: 170,
    height: 43,
};

pub static MED_LAYOUT: Rect = Rect {
    x: 0,
    y: 0,
    width: 99,
    height: 28,
};

pub static SM_LAYOUT: Rect = Rect {
    x: 0,
    y: 0,
    width: 84,
    height: 20,
};

pub enum LayoutType {
    Large,
    Med,
    Sm,
}

fn calc_term_size(cur_size: Rect) -> LayoutType {
    let cur_width = cur_size.width;
    let cur_height = cur_size.height;

    if cur_width <= SM_LAYOUT.width {
        LayoutType::Sm
    } else if cur_width <= MED_LAYOUT.width {
        LayoutType::Med
    } else {
        LayoutType::Large
    }
}

pub fn get_layout_chunks(cur_size: Rect) -> Vec<Rect> {
    let layout_type = calc_term_size(cur_size);
    match layout_type {
        LayoutType::Large => Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .margin(3)
            .split(cur_size),
        LayoutType::Med => {
            panic!("Med layout not implemented yet");
            // Layout::default().constraints([].as_ref()).split(cur_size)
        }
        LayoutType::Sm => {
            panic!("Sm layout not implemented yet");
            // Layout::default().constraints([].as_ref()).split(cur_size)
        }
    }
}

pub fn default_block() -> Block<'static> {
    let theme = Theme::new();
    Block::default()
        .style(theme.block_style)
        .border_style(theme.border_style)
        .borders(Borders::ALL)
        .border_type(theme.border_type)
}

pub fn centered_vertical_chunk(layout_chunk: Rect) -> Rect {
    let default_1_3_ratio = generate_basic_1_3_layout();
    default_1_3_ratio.split(layout_chunk)[1]
}

pub fn centered_horizontal_chunk(layout_chunk: Rect) -> Rect {
    let default_1_3_ratio = generate_basic_1_3_layout();
    default_1_3_ratio
        .direction(Direction::Horizontal)
        .split(layout_chunk)[1]
}

pub fn generate_basic_1_3_layout() -> Layout {
    Layout::default().constraints(
        [
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]
        .as_ref(),
    )
}
