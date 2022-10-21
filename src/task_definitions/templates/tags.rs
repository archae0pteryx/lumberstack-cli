use serde::{Deserialize, Serialize};

use std::fmt::{self, Display};

use crate::app_config::AppConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskTag {
    Clone,
    Create,
    Auth,
    Prisma,
    Pages,
    Github,
    Templates,
    Layouts,
    Generate,
    Tailwind,
    Playwright,
    Heroku,
}

impl Display for TaskTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskTag::Clone => write!(f, "init"),
            TaskTag::Create => write!(f, "create"),
            TaskTag::Auth => write!(f, "auth"),
            TaskTag::Prisma => write!(f, "prisma"),
            TaskTag::Pages => write!(f, "pages"),
            TaskTag::Github => write!(f, "github"),
            TaskTag::Templates => write!(f, "templates"),
            TaskTag::Layouts => write!(f, "layouts"),
            TaskTag::Generate => write!(f, "generate"),
            TaskTag::Tailwind => write!(f, "tailwind"),
            TaskTag::Playwright => write!(f, "playwright"),
            TaskTag::Heroku => write!(f, "heroku"),
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
