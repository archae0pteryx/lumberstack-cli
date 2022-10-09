#![allow(unused)]
use std::fmt::{self, Display};

#[derive(Clone)]
pub enum TaskTag {
    Init,
    Create,
}

impl Display for TaskTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskTag::Init => write!(f, "init"),
            TaskTag::Create => write!(f, "create"),
        }
    }
}

pub fn should_task_run(this_tag: &TaskTag, tags: &Option<Vec<String>>) -> bool {
    if let Some(t) = tags {
        return t.contains(&this_tag.to_string()) || t.is_empty();
    }
    return true;
}
