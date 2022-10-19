use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use crate::app_config::{AppConfig, TEMPLATE_TOKEN_REGEX};

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

lazy_static! {
    static ref TOKEN_REGEX: Regex = Regex::new(TEMPLATE_TOKEN_REGEX).unwrap();
    static ref TOKEN_TAGS_REGEX: Regex = Regex::new(r#"tags\((?P<tags>.*)\)"#).unwrap();
    static ref TOKEN_REPLACE_REGEX: Regex = Regex::new(r#"replace.*\((?P<replace>.*)\)"#).unwrap();
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

#[derive(Debug, Clone)]
pub struct Symbols;

impl Symbols {
    pub fn extract_replacers_from_file(file_str: &String) -> HashMap<String, String> {
        let mut var_map: HashMap<String, String> = HashMap::new();
        for line in file_str.lines() {
            var_map = Self::capture_and_collect(var_map, line);
        }
        var_map
    }

    fn capture_and_collect(
        mut var_map: HashMap<String, String>,
        line: &str,
    ) -> HashMap<String, String> {
        let replace_method_capture = TOKEN_REPLACE_REGEX.captures(line);
        if let Some(cap) = replace_method_capture {
            let replace_group = cap.name("replace");
            if let Some(rg) = replace_group {
                let extracted_replace_strings = Self::split_arr(rg.as_str().trim().to_string());

                extracted_replace_strings.iter().for_each(|key_value| {
                    let kv: Vec<&str> = key_value.split(':').collect();
                    let key = kv[0].to_string();
                    let value = kv[1].to_string();
                    var_map.insert(key, value);
                });
            }
        }
        var_map
    }

    pub fn parse_tags(file_str: &String) -> Vec<String> {
        for line in file_str.lines() {
            let tags_capture = TOKEN_TAGS_REGEX.captures(line);
            if let Some(cap) = tags_capture {
                let tags_group = cap.name("tags");
                if let Some(tg) = tags_group {
                    let extracted_tags = Self::split_arr(tg.as_str().trim().to_string());
                    return extracted_tags;
                }
            }
        }
        vec![]
    }

    fn split_arr(arr: String) -> Vec<String> {
        arr.split(',').map(str::to_string).collect()
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_gathers_template_files() {
//         assert!(true);
//     }
// }
