use serde::{Deserialize, Serialize};

use super::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]

pub struct FindTask {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub find: Find,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub register: String,
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
    pub fn new(name: &str) -> FindTask {
        FindTask {
            name: name.to_string(),
            find: Find::default(),
            register: String::new(),
        }
    }

    pub fn register(self: &Self, register: &str) -> FindTask {
        let mut new_task = self.clone();
        new_task.register = register.to_string();
        return new_task;
    }

    pub fn file_type(self: &Self, file_type: &str) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.file_type = file_type.to_string();
        return new_task;
    }

    pub fn paths(self: &Self, paths: &Option<String>) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.paths = paths.clone().unwrap_or_default();
        return new_task;
    }

    pub fn recurse(self: &Self, recurse: &str) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.recurse = recurse.to_string();
        return new_task;
    }

    pub fn exclude(self: &Self, exclude: &str) -> FindTask {
        let new_excludes = [self.find.excludes.clone(), vec![exclude.to_string()]].concat();
        let mut new_task = self.clone();
        new_task.find.excludes = new_excludes;
        return new_task;
    }

    pub fn hidden(self: &Self, hidden: &str) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.hidden = hidden.to_string();
        return new_task;
    }

    pub fn contains(self: &Self, contains: &str) -> FindTask {
        let mut new_task = self.clone();
        new_task.find.contains = contains.to_string();
        return new_task;
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::Find(self.clone())
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
            .paths(&Some("boing".to_string()))
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
        assert!(matches!(built, TaskType::Find { .. }));
    }
}
