use super::{symbols::Symbols, template_io::TemplateIO};
use crate::{app_config::AppConfig, system::file_io::FileIO};
use anyhow::Result;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use std::path::PathBuf;
use std::str::FromStr;
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(
    Serialize, Deserialize, Debug, Clone, Sequence, EnumString, AsRefStr, EnumIter, PartialEq,
)]
pub enum TaskTag {
    Clone,
    Create,
    Auth,
    Prisma,
    Pages,
    Github,
    Templates,
    Layouts,
    Tailwind,
    Playwright,
    Heroku,
    None,
    All,
}

pub fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn tag_to_str(tag: &TaskTag) -> String {
    TaskTag::as_ref(tag).to_string()
}

pub fn opt_tags_to_vec(tags: Option<Vec<String>>) -> Vec<TaskTag> {
    tags.unwrap_or_default()
        .into_iter()
        .map(|t| TaskTag::from_str(&t))
        .filter_map(|t| t.ok())
        .collect::<Vec<_>>()
}

pub fn should_task_run(this_tag: &TaskTag, app_config: &AppConfig) -> bool {
    let tags = app_config.tags.to_owned();
    let skip_tags = &app_config.skip_tags.to_owned();

    if tags.contains(&TaskTag::All)
        || tags.contains(this_tag)
        || (tags.is_empty() && skip_tags.is_empty())
    {
        return true;
    }

    if skip_tags.contains(this_tag) {
        return false;
    }

    true
}

pub fn get_all_tags(map_file: &str) -> Result<Vec<(Option<TaskTag>, String)>> {
    let combined_templates = TemplateIO::gather_all_template_paths(map_file)?;
    let tag_strs = all_tags_from_files(combined_templates);
    let tags = tag_strs
        .into_iter()
        .map(|t| {
            let tc_string = title_case(&t);
            let tag_match = TaskTag::from_str(tc_string.as_str());
            if let Ok(tag_match) = tag_match {
                return (Some(tag_match), tc_string);
            }
            return (None, tc_string);
        })
        .collect::<Vec<_>>();
    Ok(tags)
}

fn all_tags_from_files(template_paths: Vec<PathBuf>) -> Vec<String> {
    template_paths
        .iter()
        .flat_map(|path| {
            let file_str = FileIO::read_or_skip(&path);
            Symbols::get_tags(&file_str.unwrap_or_default())
        })
        .collect::<Vec<_>>()
}
