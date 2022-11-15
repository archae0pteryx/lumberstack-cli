use crate::{
    app_config::AppConfig,
    task_definitions::templates::tags::{TagData, TaskTag},
};
use std::time::{Duration, Instant};

use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::ListState,
};

use super::screens::{
    generate_all::generate_screen_menu, home::home_screen_menu, progress::progress_menu,
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
    pub progress_screen_menu: Vec<&'static str>,
    pub tag_select_data: TagSelectData,
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
            navigation_stack: vec![Screen::TagSelect],
            term_size: Rect::default(),
            tasks_to_run: vec![],
            ready_to_execute: false,
            menu_list_state: ListState::default(),
            home_screen_menu: home_screen_menu(),
            generate_screen_menu: generate_screen_menu(),
            progress_screen_menu: progress_menu(),
            tag_select_data: TagSelectData::new(vec![]),
        }
    }
}

impl App {
    pub fn new(app_config: AppConfig) -> App {
        App {
            app_config: app_config.clone(),
            tag_select_data: TagSelectData::new(app_config.all_found_tags),
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

pub trait Selectable {
    fn next_item(&mut self) {
        let len = self.get_items().len();
        let idx = match self.get_selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.set_selected(Some(idx))
    }
    fn prev_item(&mut self) {
        let len = self.get_items().len();
        let i = match self.get_selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.set_selected(Some(i));
    }
    fn get_items(&self) -> Vec<String>;
    fn get_selected(&self) -> Option<usize>;
    fn set_selected(&mut self, idx: Option<usize>);
}

#[derive(Debug, Clone)]
pub struct TagSelectData {
    pub state: ListState,
    pub list_items: Vec<TagListItem>,
    pub items: Vec<String>,
    pub tag_columns: Vec<TagColumn>,
    pub cur_column: usize,
}

#[derive(Debug, Clone)]
pub struct TagColumn {
    pub state: ListState,
    pub items: Vec<TagListItem>,
}

#[derive(Debug, Clone)]
pub struct TagListItem {
    pub idx: usize,
    pub tag: TaskTag,
    pub name: String,
    pub is_selected: bool,
}

impl Selectable for TagSelectData {
    fn get_items(&self) -> Vec<String> {
        self.items.clone()
    }

    fn get_selected(&self) -> Option<usize> {
        let cur_column = self.cur_column;
        self.tag_columns[cur_column].state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        let cur_column = self.cur_column;
        self.tag_columns[cur_column].state.select(idx);
    }
}

impl TagSelectData {
    fn new(tags: Vec<TagData>) -> Self {
        let tag_columns = Self::create_columns(tags.clone());
        TagSelectData {
            state: ListState::default(),
            list_items: tags
                .clone()
                .into_iter()
                .enumerate()
                .map(|(idx, tag)| TagListItem {
                    idx,
                    tag: tag.tag,
                    name: tag.name,
                    is_selected: true,
                })
                .collect(),
            items: tags.into_iter().map(|d| d.name).collect(),
            tag_columns,
            cur_column: 0,
        }
    }

    pub fn increase_column(&mut self) {
        if self.cur_column < self.tag_columns.len() - 1 {
            let cur_idx = self.get_selected();
            self.set_selected(None);
            self.cur_column += 1;
            self.set_selected(cur_idx);
        }
    }

    pub fn decrease_column(&mut self) {
        if self.cur_column > 0 {
            let cur_idx = self.get_selected();
            self.set_selected(None);
            self.cur_column -= 1;
            self.set_selected(cur_idx);
        }
    }

    pub fn toggle_selected(&mut self) {
        let cur_state_selected = self.state.selected();
        if let Some(idx) = cur_state_selected {
            let item = self.list_items.get_mut(idx).unwrap();
            item.is_selected = !item.is_selected;
        }
    }

    fn create_columns(tags: Vec<TagData>) -> Vec<TagColumn> {
        let mut tag_columns = vec![];
        let mut col = TagColumn {
            state: ListState::default(),
            items: vec![],
        };
        for (idx, tag) in tags.into_iter().enumerate() {
            col.items.push(TagListItem {
                idx,
                tag: tag.tag,
                name: tag.name,
                is_selected: true,
            });
            if col.items.len() == 10 {
                tag_columns.push(col);
                col = TagColumn {
                    state: ListState::default(),
                    items: vec![],
                };
            }
        }
        tag_columns.push(col);
        tag_columns
    }
}
