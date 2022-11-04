use std::time::{Duration, Instant};

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

pub struct EventClock {
    pub tick_rate: Duration,
    pub last_tick: Instant,
}

pub struct App {
    pub app_config: Box<AppConfig>,
    pub theme: Theme,
    pub should_quit: bool,
    pub navigation_stack: Vec<Screen>,
    pub term_size: Rect,
    pub list_state: ListState,
    pub is_first_render: bool,
    pub clock: EventClock,
}

impl Default for App {
    fn default() -> Self {
        App {
            app_config: Box::new(AppConfig::default()),
            is_first_render: true,
            clock: EventClock {
                tick_rate: Duration::from_millis(50),
                last_tick: Instant::now(),
            },
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
            list_state: ListState::default(),
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

    pub fn push_route(&mut self, route: Screen) {
        self.navigation_stack.push(route);
    }

    pub fn get_timeout(&self) -> Duration {
        self.clock
            .tick_rate
            .checked_sub(self.clock.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0))
    }

    pub fn tick(&mut self) {
        if self.clock.last_tick.elapsed() >= self.clock.tick_rate {
            self.clock.last_tick = Instant::now();
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
