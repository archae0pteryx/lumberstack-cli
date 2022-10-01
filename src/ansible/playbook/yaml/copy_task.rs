use serde::{Deserialize, Serialize};

use super::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CopyTask {
    pub name: String,
    pub copy: CopyArgs,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CopyArgs {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub src: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub dest: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub content: String,
}

impl CopyTask {
    pub fn new(name: &str) -> CopyTask {
        CopyTask {
            name: name.to_string(),
            copy: CopyArgs::default(),
        }
    }

    #[allow(dead_code)]
    pub fn src(self: &Self, src: &str) -> CopyTask {
        let mut new_task = self.clone();
        new_task.copy.src = src.to_string();
        return new_task;
    }

    pub fn dest(self: &Self, dest: &str) -> CopyTask {
        let mut new_task = self.clone();
        new_task.copy.dest = dest.to_string();
        return new_task;
    }

    pub fn content(self: &Self, content: &str) -> CopyTask {
        let mut new_task = self.clone();
        new_task.copy.content = content.to_string();
        return new_task;
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::Copy(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_copy_task() {
        let actual = CopyTask::new("foo").src("bar").dest("baz");
        assert_eq!(actual.copy.src, "bar");
        assert_eq!(actual.copy.dest, "baz");
        assert_eq!(actual.name, "foo");
    }
}