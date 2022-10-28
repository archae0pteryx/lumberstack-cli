use crate::app_config::AppConfig;

use tui::{layout::Rect, style::Color};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RouteId {
    Home,
    GenerateAll,
    TagSelect,
    Quit,
}
#[derive(Debug)]
pub struct LinkItem {
    pub id: RouteId,
    pub text: String,
    pub index: usize,
    pub to: RouteId,
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
    pub navigation_stack: Vec<RouteId>,
    pub term_size: Rect,
    pub main_menu_items: Vec<LinkItem>,
    pub main_menu_current_index: usize,
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
            navigation_stack: vec![RouteId::Home],
            term_size: Rect::default(),
            main_menu_items: vec![
                LinkItem {
                    text: "Generate All".to_string(),
                    index: 0,
                    to: RouteId::GenerateAll,
                    id: RouteId::GenerateAll,
                },
                LinkItem {
                    text: "Select Tags".to_string(),
                    index: 1,
                    to: RouteId::TagSelect,
                    id: RouteId::TagSelect,
                },
                LinkItem {
                    text: "Quit".to_string(),
                    index: 2,
                    to: RouteId::Quit,
                    id: RouteId::Quit,
                },
            ],
            main_menu_current_index: 0,
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

    pub fn get_current_route(&self) -> &RouteId {
        self.navigation_stack.last().unwrap_or(&RouteId::Home)
    }

    pub fn next_item(&mut self) {
        if self.get_current_route() == &RouteId::Home {
            let menu_len = self.main_menu_items.len();
            let next_index = self.main_menu_current_index + 1;
            if next_index <= menu_len {
                self.main_menu_current_index = next_index;
            }
        }
    }

    pub fn prev_item(&mut self) {
        if self.get_current_route() == &RouteId::Home && self.main_menu_current_index > 0 {
            self.main_menu_current_index -= 1;
        }
    }

    pub fn handle_submit(&mut self) {
        if self.get_current_route() == &RouteId::Home {
            let idx = self.main_menu_current_index;
            let link_to = &self.main_menu_items[idx].to;
            if link_to.eq(&RouteId::Quit) {
                self.should_quit = true;
            } else {
                self.navigation_stack.push(link_to.clone());
            }
        }
    }

    pub fn on_tick(&self) {
        // handle tick
    }
}
