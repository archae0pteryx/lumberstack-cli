use tui::widgets::ListState;

use crate::{task_definitions::templates::tags::{TaskTag, TagData}, ui::app::Selectable};


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
    pub fn new(tags: Vec<TagData>) -> Self {
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
        let cur_col = self.cur_column;
        let cur_state = &self.tag_columns[cur_col].state;
        let cur_state_selected = cur_state.selected();
        let cur_items = &mut self.tag_columns[cur_col].items;

        if let Some(idx) = cur_state_selected {
            let item = &mut cur_items[idx];
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
            if col.items.len() == 7 {
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
