use serde::{Deserialize, Serialize};

use std::fmt::{self, Display};

use crate::app_config::AppConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskTag {
    Init,
    Create,
    Auth,
    Prisma,
    Docker,
    Pages,
    Markdown,
    Github,
    Parse,
    Templates,
    Layouts,
    Generate,
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
            TaskTag::Parse => write!(f, "parse"),
            TaskTag::Templates => write!(f, "templates"),
            TaskTag::Layouts => write!(f, "layouts"),
            TaskTag::Generate => write!(f, "generate"),
        }
    }
}

pub fn should_task_run(this_tag: &TaskTag, app_config: &AppConfig) -> bool {
    let tags = app_config.tags.to_owned();
    let skip_tags = &app_config.skip_tags.to_owned();
    if let Some(t) = tags {
        return t.contains(&this_tag.to_string()) || t.is_empty();
    }
    if let Some(st) = skip_tags {
        let has_skip_tag = st.contains(&this_tag.to_string());
        return !has_skip_tag;
    }

    true
}
