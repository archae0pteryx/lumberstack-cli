use serde::{Deserialize, Serialize};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]

pub struct FindTask {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub find: Find,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub register: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]

pub struct Find {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub paths: String, // TODO: make me a vec
    #[serde(skip_serializing_if = "String::is_empty")]
    pub file_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excludes: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub recurse: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub hidden: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub contains: String,
}

impl FindTask {
    pub fn new<S: AsRef<str>>(name: S) -> FindTask {
        FindTask {
            name: name.as_ref().to_string(),
            find: Find::default(),
            register: String::new(),
            tags: None
        }
    }

    pub fn register<S: AsRef<str>>(&self, register: S) -> FindTask {
        let mut new_task = self.clone();
        new_task.register = register.as_ref().to_owned();
        new_task
    }

    pub fn file_type<S: AsRef<str>>(&self, file_type: S) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.file_type = file_type.as_ref().to_string();
        new_task
    }

    pub fn paths<S: AsRef<str>>(&self, paths: S) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.paths = paths.as_ref().to_string();
        new_task
    }

    pub fn recurse<S: AsRef<str>>(&self, recurse: S) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.recurse = recurse.as_ref().to_string();
        new_task
    }

    pub fn exclude<S: AsRef<str>>(&self, exclude: S) -> FindTask {
        let new_excludes = [
            self.find.excludes.clone(),
            vec![exclude.as_ref().to_string()],
        ]
        .concat();
        let mut new_task = self.clone();
        new_task.find.excludes = new_excludes;
        new_task
    }

    pub fn hidden<S: AsRef<str>>(&self, hidden: S) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.hidden = hidden.as_ref().to_string();
        new_task
    }

    pub fn contains<S: AsRef<str>>(&self, contains: S) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.contains = contains.as_ref().to_string();
        new_task
    }

    pub fn tags(&self, tags: Option<Vec<String>>) -> FindTask {
        let mut new_task = self.clone();
        new_task.tags = tags;
        new_task
    }

    pub fn build(&self) -> DefinedTask {
        DefinedTask::Find(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_find_task() {
        let actual = FindTask::new("foo")
            .register("bar")
            .file_type("baz")
            .paths("boing")
            .recurse("snap")
            .exclude("crackle")
            .hidden("mitch")
            .contains("pop");

        assert_eq!(actual.name, "foo");
        assert_eq!(actual.register, "bar");
        assert_eq!(actual.find.file_type, "baz");
        assert_eq!(actual.find.paths, "boing");
        assert_eq!(actual.find.recurse, "snap");
        assert_eq!(actual.find.excludes[0], "crackle");
        assert_eq!(actual.find.hidden, "mitch");
        assert_eq!(actual.find.contains, "pop");

        let built = actual.build();
        assert!(matches!(built, DefinedTask::Find { .. }));
    }
}
