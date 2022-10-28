#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FileTask {
    pub name: String,
    pub file: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct File {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
}

impl FileTask {
    pub fn new<S: AsRef<str>>(name: S) -> FileTask {
        FileTask {
            name: name.as_ref().to_string(),
            file: File {
                path: String::new(),
                state: String::new(),
            },
            tags: None,
        }
    }
    pub fn path<S: AsRef<str>>(&self, path: S) -> FileTask {
        let mut new_task = self.clone();
        new_task.file.path = path.as_ref().to_string();
        new_task
    }

    pub fn state<S: AsRef<str>>(&self, state: S) -> FileTask {
        let mut new_task = self.clone();
        new_task.file.state = state.as_ref().to_string();
        new_task
    }

    pub fn build(&self) -> DefinedTask {
        DefinedTask::File(self.clone())
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
        assert!(matches!(built, DefinedTask::File { .. }));
    }
}
