use crate::app_config::AppConfig;

use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::ListState,
};


#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Screen {
    Home,
    GenerateAll,
    TagSelect,
    Quit,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub list_highlight: Style,
    pub list_item: Style,
}

pub struct App {
    pub app_config: Box<AppConfig>,
    pub theme: Theme,
    pub should_quit: bool,
    pub navigation_stack: Vec<Screen>,
    pub term_size: Rect,
    pub list_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            app_config: Box::new(AppConfig::default()),
            theme: Theme {
                primary: Color::White,
                secondary: Color::LightYellow,
                tertiary: Color::LightGreen,
                list_highlight: Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
                list_item: Style::default().fg(Color::Red),
            },
            should_quit: false,
            navigation_stack: vec![Screen::Home],
            term_size: Rect::default(),
            list_state,
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

    pub fn current_route(&self) -> &Screen {
        self.navigation_stack.last().unwrap_or(&Screen::Home)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
