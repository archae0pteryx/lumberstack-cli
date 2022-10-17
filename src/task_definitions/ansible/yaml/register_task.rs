use serde::{Deserialize, Serialize};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct RegisterTask {
    pub name: String,
    pub stat: StatPath,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub register: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct StatPath {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
}

impl RegisterTask {
    pub fn new<S: AsRef<str>>(name: S) -> RegisterTask {
        RegisterTask {
            name: name.as_ref().to_string(),
            stat: StatPath::default(),
            register: String::new(),
            tags: None
        }
    }

    pub fn register<S: AsRef<str>>(&self, register: S) -> RegisterTask {
        let mut new_task = self.clone();
        new_task.register = register.as_ref().to_string();
        return new_task;
    }

    pub fn stat_path<S: AsRef<str>>(&self, path: S) -> RegisterTask {
        let mut new_task = self.clone();
        new_task.stat.path = path.as_ref().to_string();
        return new_task;
    }

    pub fn tags(&self, tags: Option<Vec<String>>) -> RegisterTask {
        let mut new_task = self.clone();
        new_task.tags = tags;
        return new_task;

    }
    pub fn build(&self) -> DefinedTask {
        DefinedTask::Register(self.clone())

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_register_task() {
        let actual = RegisterTask::new("foo")
            .register("bar")
            .stat_path("baz");

        assert_eq!(actual.name, "foo");
        assert_eq!(actual.register, "bar");
        assert_eq!(actual.stat.path, "baz");

        let built = actual.build();
        assert!(matches!(built, DefinedTask::Register { .. }));
    }
}
