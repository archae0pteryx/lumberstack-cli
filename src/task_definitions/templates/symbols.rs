use lazy_static::lazy_static;
use regex::{Match, Regex};

use std::collections::HashMap;

use crate::app_config::TEMPLATE_TOKEN_REGEX;

lazy_static! {
    static ref TOKEN_REGEX: Regex = Regex::new(TEMPLATE_TOKEN_REGEX).unwrap();
    static ref TOKEN_TAGS_REGEX: Regex = Regex::new(r#"tags\((?P<tags>.*)\)"#).unwrap();
    static ref TOKEN_REPLACE_REGEX: Regex = Regex::new(r#"replace.*\((?P<replace>.*)\)"#).unwrap();
}

fn capture_replace_symbol(line: &str) -> Option<Match> {
    let replace_method_capture = TOKEN_REPLACE_REGEX.captures(line);
    if let Some(cap) = replace_method_capture {
        let replace_group = cap.name("replace");
        return replace_group;
    }
    None
}

fn capture_tag_symbol(line: &str) -> Option<Match> {
    let tags_capture = TOKEN_TAGS_REGEX.captures(line);
    if let Some(cap) = tags_capture {
        let tags_group = cap.name("tags");
        return tags_group;
    }
    None
}

#[derive(Debug, Clone)]
pub struct Symbols;

impl Symbols {
    pub fn get_replacers(file_str: &str) -> HashMap<String, String> {
        let mut var_map: HashMap<String, String> = HashMap::new();
        for line in file_str.lines() {
            var_map = Self::capture_and_collect(var_map, line);
        }
        var_map
    }

    pub fn get_tags(file_str: &str) -> Vec<String> {
        for line in file_str.lines() {
            let tags_group = capture_tag_symbol(line);
            if let Some(tg) = tags_group {
                let extracted_tags = Self::split_arr(tg.as_str().trim().to_string());
                return extracted_tags;
            }
        }
        vec![]
    }

    fn capture_and_collect(
        mut var_map: HashMap<String, String>,
        line: &str,
    ) -> HashMap<String, String> {
        let replace_group = capture_replace_symbol(line);
        if let Some(rg) = replace_group {
            let extracted_replace_strings = Self::split_arr(rg.as_str().trim().to_string());

            extracted_replace_strings.iter().for_each(|key_value| {
                let kv: Vec<&str> = key_value.split(':').collect();
                let key = kv[0].to_string();
                let value = kv[1].to_string();
                var_map.insert(key, value);
            });
        }
        var_map
    }

    fn split_arr(arr: String) -> Vec<String> {
        arr.split(',').map(str::to_string).collect()
    }

    pub fn remove_symbols(replaced_content: &str) -> String {
        replaced_content
            .lines()
            .filter(|&line| !TOKEN_REGEX.is_match(line))
            .collect::<Vec<_>>()
            .join("\n")
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
