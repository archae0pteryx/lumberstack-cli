use crate::{app_config::AppConfig, task_definitions::templates::tags::TaskTag};
use std::time::{Duration, Instant};

use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::ListState,
};

use super::screens::{
    generate_all::generate_screen_menu, home::home_screen_menu, progress::progress_menu,
    tag_select::tag_select_menu,
};

#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Screen {
    Home,
    GenerateAll,
    TagSelect,
    Progress,
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
    pub app_config: AppConfig,
    pub theme: Theme,
    pub should_quit: bool,
    pub navigation_stack: Vec<Screen>,
    pub term_size: Rect,
    pub menu_list_state: ListState,
    pub is_first_render: bool,
    pub clock: EventClock,
    pub tasks_to_run: Vec<TaskTag>,
    pub ready_to_execute: bool,
    pub home_screen_menu: Vec<&'static str>,
    pub generate_screen_menu: Vec<&'static str>,
    pub select_tag_screen_menu: Vec<&'static str>,
    pub progress_screen_menu: Vec<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        App {
            app_config: AppConfig::default(),
            is_first_render: true,
            clock: EventClock {
                tick_rate: Duration::from_millis(10),
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
            tasks_to_run: vec![],
            ready_to_execute: false,
            menu_list_state: ListState::default(),
            home_screen_menu: home_screen_menu(),
            generate_screen_menu: generate_screen_menu(),
            select_tag_screen_menu: tag_select_menu(),
            progress_screen_menu: progress_menu(),
        }
    }
}

impl App {
    pub fn new(app_config: AppConfig) -> App {
        App {
            app_config,
            ..App::default()
        }
    }

    pub fn current_route(&self) -> &Screen {
        self.navigation_stack.last().unwrap_or(&Screen::Home)
    }

    pub fn push_route(&mut self, route: Screen) {
        self.menu_list_state.select(Some(0));
        self.navigation_stack.push(route);
    }

    pub fn pop_route(&mut self) {
        self.menu_list_state.select(Some(0));
        self.navigation_stack.pop();
    }

    // pub fn add_remove_task_to_run(&mut self, task: TaskTag) {
    //     let position = self.tasks_to_run.iter().position(|t| t == &task);
    //     match position {
    //         Some(_) => {
    //             self.tasks_to_run.remove(position.unwrap());
    //         }
    //         None => {
    //             self.tasks_to_run.push(task);
    //         }
    //     }
    // }

    pub fn next_menu_item(&mut self, len: usize) {
        let i = match self.menu_list_state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_list_state.select(Some(i));
    }

    pub fn prev_menu_item(&mut self, len: usize) {
        let i = match self.menu_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_list_state.select(Some(i));
    }

    pub fn update_on_tick(&self) {
        //
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
