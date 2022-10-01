#![allow(unused)]
use serde::{Deserialize, Serialize};

use super::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileTask {
    pub name: String,
    pub file: File,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
}

impl FileTask {
    pub fn new(name: &str) -> FileTask {
        FileTask {
            name: name.to_string(),
            file: File {
                path: String::new(),
                state: String::new(),
                // hidden: String::new()
            },
        }
    }
    pub fn path(self: &Self, path: &str) -> FileTask {
        let mut new_task = self.clone();
        new_task.file.path = path.to_string();
        return new_task;
    }

    pub fn state(self: &Self, state: &str) -> FileTask {
        let mut new_task = self.clone();
        new_task.file.state = state.to_string();
        return new_task;
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::File(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_file_task() {
        let actual = FileTask::new("foo").path("bar").state("baz");
        assert_eq!(actual.name, "foo");
        assert_eq!(actual.file.path, "bar");
        assert_eq!(actual.file.state, "baz");

        let built = actual.build();
        assert!(matches!(built, TaskType::File { .. }));
    }
}
