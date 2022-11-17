use tui::widgets::ListState;

use crate::ui::{app::Selectable, screens::Screen};

#[derive(Debug, Clone)]
pub struct HomeMenuData {
    pub state: ListState,
    pub items: Vec<(String, Screen)>,
    pub next_route: Screen,
}

impl Selectable for HomeMenuData {
    fn get_items(&self) -> Vec<String> {
        self.items.iter().cloned().map(|i| i.0).collect::<Vec<_>>()
    }

    fn get_selected(&self) -> Option<usize> {
        self.state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        self.state.select(idx);
        self.set_next_route(idx);
    }
}

impl HomeMenuData {
    pub fn new() -> Self {
        HomeMenuData {
            state: ListState::default(),
            items: vec![
                ("New Project".to_string(), Screen::NewProject),
                ("Install with defaults".to_string(), Screen::NewProject),
                ("Select modules".to_string(), Screen::TagSelect),
                ("Quit".to_string(), Screen::Quit),
            ],
            next_route: Screen::None,
        }
    }

    fn set_next_route(&mut self, idx: Option<usize>) {
        let len = &self.items.len();
        if idx.is_some() || idx.unwrap_or_default().gt(len) {
            let (_, screen) = &self.items[idx.unwrap()];
            self.next_route = screen.clone();
        }
    }

    pub fn navigate(&mut self) -> Screen {
        let nav_to = self.next_route.clone();
        self.next_route = Screen::None;
        nav_to
    }
}
