use serde::{Deserialize, Serialize};

use crate::{task_definitions::task_types::DefinedTask, lumberstack::Runnable};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CopyTask {
    pub name: String,
    pub copy: CopyArgs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>

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


impl Runnable for CopyTask {
    fn run_job(&self) {
        
    }
}


impl CopyTask {
    pub fn new<S: AsRef<str>>(name: S) -> CopyTask {
        CopyTask {
            name: name.as_ref().to_string(),
            copy: CopyArgs::default(),
            tags: None
        }
    }

    #[allow(dead_code)]
    pub fn src<S: AsRef<str>>(&self, src: S) -> CopyTask {
        let mut new_task = self.clone();
        new_task.copy.src = src.as_ref().to_string();
        return new_task;
    }

    pub fn dest<S: AsRef<str>>(&self, dest: S) -> CopyTask {
        let mut new_task = self.clone();
        new_task.copy.dest = dest.as_ref().to_string();
        return new_task;
    }

    pub fn content<S: AsRef<str>>(&self, content: S) -> CopyTask {
        let mut new_task = self.clone();
        new_task.copy.content = content.as_ref().to_string();
        return new_task;
    }

    pub fn set_tags(&self, tags: Option<Vec<String>>) -> CopyTask {
        let mut new_task = self.clone();
        new_task.tags = tags;
        return new_task;
    }

    pub fn build(&self) -> DefinedTask {
        DefinedTask::Copy(self.clone())
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
