#![allow(unused)]
use std::fmt::{self, Display};

pub trait Runnable {
    fn run_job(&self);
}


#[derive(Clone, Debug)]
pub enum TaskTag {
    Init,
    Create,
    Auth,
    Prisma,
    Docker,
    Pages,
    Markdown,
    Github,
    Parse
}

impl Display for TaskTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskTag::Init => write!(f, "init"),
            TaskTag::Create => write!(f, "create"),
            TaskTag::Auth => write!(f, "auth"),
            TaskTag::Prisma => write!(f, "prisma"),
            TaskTag::Docker => write!(f, "docker"),
            TaskTag::Pages => write!(f, "pages"),
            TaskTag::Markdown => write!(f, "markdown"),
            TaskTag::Github => write!(f, "github"),
            TaskTag::Parse => write!(f, "parse")
        }
    }
}

pub fn should_task_run(this_tag: &TaskTag, tags: &Option<Vec<String>>) -> bool {
    if let Some(t) = tags {
        return t.contains(&this_tag.to_string()) || t.is_empty();
    }
    return true;
}
