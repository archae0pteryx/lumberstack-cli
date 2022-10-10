#![allow(unused)]
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use std::{
    fmt::{self, Display},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{TEMPLATE_TOKEN_REGEX};

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
    Parse,
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
        }
    }
}

pub fn should_task_run(this_tag: &TaskTag, tags: &Option<Vec<String>>) -> bool {
    if let Some(t) = tags {
        return t.contains(&this_tag.to_string()) || t.is_empty();
    }
    return true;
}

pub type Tags = Vec<String>;

#[derive(Debug, Clone)]
pub struct ReplaceVars {
    line_num: usize,
    key: String,
    value: String,
}

pub struct Symbol;

impl Symbol {
    pub fn new(template_path: PathBuf) -> (Option<Tags>, Option<Vec<ReplaceVars>>) {
        let tags = Self::process_tags(&template_path);
        let replace_vars = Self::process_replace_vars(&template_path);
        (tags, replace_vars)
    }

    fn process_replace_vars(template_path: &PathBuf) -> Option<Vec<ReplaceVars>> {
        let f = File::open(template_path).expect("Error getting path buffer");
        let reader = BufReader::new(f);
        let to_replace = reader
            .lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .filter_map(|(n, l)| Self::extract_replacers(n, l))
            .flatten()
            .collect::<Vec<ReplaceVars>>();

        if to_replace.is_empty() {
            return None;
        }

        Some(to_replace)
    }

    fn process_tags(template_path: &PathBuf) -> Option<Tags> {
        let f = File::open(template_path).expect("Error getting path buffer");
        let reader = BufReader::new(f);
        let tag_vec = reader
            .lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .filter(|(n, l)| n.lt(&3))
            .filter_map(|(n, l)| {
                let tag_method_capture = TOKEN_TAGS_REGEX.captures(l.as_str());
                let tags = Self::extract_tags(tag_method_capture);
                tags
            })
            .flatten()
            .collect::<Vec<_>>();

        if tag_vec.is_empty() {
            return None;
        }
        return Some(tag_vec);
    }

    fn extract_tags(tag_cap: Option<Captures>) -> Option<Tags> {
        if let Some(cap) = tag_cap {
            let tag_group = cap.name("tags");
            if let Some(tg) = tag_group {
                let group_string = tg.as_str().trim().to_string();
                return Some(Self::split_arr(group_string));
            }
            return None;
        }
        return None;
    }

    fn split_arr(arr: String) -> Vec<String> {
        arr.split(",").map(str::to_string).collect()
    }

    fn extract_replacers(line_num: usize, line: String) -> Option<Vec<ReplaceVars>> {
        let replace_method_capture = TOKEN_REPLACE_REGEX.captures(line.as_str());
        if let Some(cap) = replace_method_capture {
            let replace_group = cap.name("replace");
            if let Some(rg) = replace_group {
                let mut replace_vars = Vec::new();
                let vec_of_kv = Self::split_arr(rg.as_str().trim().to_string());
                vec_of_kv.iter().for_each(|kv| {
                    let (key, value) = kv.split_once(":").unwrap();
                    let replace_var = ReplaceVars {
                        line_num,
                        key: key.to_string(),
                        value: value.to_string(),
                    };
                    replace_vars.push(replace_var);
                });
                return Some(replace_vars);
            }
            return None;
        }
        return None;
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
