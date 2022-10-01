use serde::{Deserialize, Serialize};

use super::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct StatPath {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct RegisterTask {
    pub name: String,
    pub stat: StatPath,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub register: String,
}

impl RegisterTask {
    pub fn new(name: &str) -> RegisterTask {
        RegisterTask {
            name: name.to_string(),
            stat: StatPath::default(),
            register: String::new(),
        }
    }

    pub fn register(self: &Self, register: &str) -> RegisterTask {
        let mut new_task = self.clone();
        new_task.register = register.to_string();
        return new_task;
    }

    #[allow(dead_code)]
    pub fn stat_path(self: &Self, path: &str) -> RegisterTask {
        let mut new_task = self.clone();
        new_task.stat.path = path.to_string();
        return new_task;
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::Register(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_register_task() {
        let actual = RegisterTask::new("foo").register("bar").stat_path("baz");

        assert_eq!(actual.name, "foo");
        assert_eq!(actual.register, "bar");
        assert_eq!(actual.stat.path, "baz");

        let built = actual.build();
        assert!(matches!(built, TaskType::Register { .. }));
    }
}
