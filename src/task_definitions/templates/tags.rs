use serde::{Deserialize, Serialize};

use crate::{app_config::AppConfig, system::file_io::FileIO};
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
use std::{
    fmt::{self, Display},
    vec, path::PathBuf,
};
use anyhow::Result;
use super::{template_io::TemplateIO, symbols::Symbols};

#[derive(Serialize, Deserialize, Debug, Clone, Sequence)]
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
    Quit,
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
            TaskTag::Quit => write!(f, "quit"),
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

pub fn extract_all_tags(map_file: &str) -> Result<Vec<String>> {
    let combined_templates = TemplateIO::gather_all_template_paths(map_file)?;
    Ok(extract_tags_from_paths(combined_templates))
}

fn extract_tags_from_paths(template_paths: Vec<PathBuf>) -> Vec<String> {
    template_paths
        .iter()
        .map(|path| {
            let file_str = FileIO::read_or_skip(&path)
                .expect(format!("Error reading: {:?}", &path.display()).as_str());
            let tags = Symbols::get_tags(&file_str);
            return tags;
        })
        .flatten()
        .collect::<Vec<_>>()
}
