use std::{
    fmt::{self, Display},
    io::Stdout,
};

use crate::{
    app_config::{self, AppConfig},
    task_definitions::templates::tags::TaskTag,
};
use anyhow::Result;
use enum_iterator::{all, Sequence};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    style::Color,
    text::Text,
    widgets::ListState,
    Frame, Terminal,
};

use super::home_view::draw_home;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn handle_selection(&mut self) {
        let selected = self.state.selected();
        match selected {
            Some(s) => {
                dbg!(&self.state);
            }
            None => {
                println!("None selected");
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogContext {
    PlaylistWindow,
    PlaylistSearch,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    GenerateAll,
    Empty,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
}

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Empty,
    hovered_block: ActiveBlock::GenerateAll,
};

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

pub struct MainMenuItem {
    pub id: usize,
    pub text: String,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub selected: Color,
    pub hovered: Color,
    pub inactive: Color,
}

pub struct App {
    pub app_config: Box<AppConfig>,
    pub theme: Theme,
    pub should_quit: bool,
    pub navigation_stack: Vec<Route>,
    pub size: Rect,
    pub main_menu: Vec<MainMenuItem>,
    pub main_menu_list_index: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            app_config: Box::new(AppConfig::default()),
            theme: Theme {
                primary: Color::White,
                secondary: Color::LightYellow,
                tertiary: Color::LightGreen,
                selected: Color::Green,
                hovered: Color::Blue,
                inactive: Color::Gray,
            },
            should_quit: false,
            navigation_stack: vec![DEFAULT_ROUTE],
            size: Rect::default(),
            main_menu: vec![
                MainMenuItem {
                    id: 0,
                    text: "Generate All".to_string(),
                },
                MainMenuItem {
                    id: 1,
                    text: "Quit".to_string(),
                },
            ],
            main_menu_list_index: 0,
        }
    }
}

impl App {
    pub fn new(app_config: Box<AppConfig>) -> App {
        App {
            app_config,
            ..App::default()
        }
    }

    pub fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    pub fn on_tick(&self) {
        // handle tick
    }
}
