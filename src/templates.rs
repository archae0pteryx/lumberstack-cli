use crate::manifest::Manifest;
use crate::{TEMPLATE_SANITIZE_REGEX, TEMPLATE_TOKEN_REGEX};
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use regex::{Captures, RegexSet, RegexSetBuilder};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Rev;
use std::path::Path;
use std::path::PathBuf;

pub type TemplateTags = Vec<String>;

#[derive(Debug)]
pub struct TemplateToken {
    pub line_number: usize,
    pub token: String,
}

lazy_static! {
    // static ref TEMPLATE_REGEX: RegexSet = RegexSetBuilder::new(&[TEMPLATE_TOKEN_REGEX])
    //     .case_insensitive(true)
    //     .build()
    //     .unwrap();

    // static ref TOKEN_REGEX: regex::Regex = regex::Regex::new(TEMPLATE_SANITIZE_REGEX).unwrap();
    static ref TOKEN_REGEX: regex::Regex = regex::Regex::new(r#"(//\*|//|#|<!--)\stemplate\[((?P<method>[^\]]+))\]"#).unwrap();
    // static ref SANITIZE_REGEX: regex::Regex = regex::Regex::new(TEMPLATE_SANITIZE_REGEX).unwrap();
    // static ref EXTRACT_METHOD_REGEX: regex::Regex = regex::Regex::new(r#"(\[.*\])"#).unwrap();
}

pub struct Templates;

impl Templates {
    pub fn collect_templates(manifest: Manifest, spinner: &ProgressBar) -> Vec<String> {
        let paths = Self::load_paths_vec(manifest.clone());
        for path in paths {
            let dest_path = Self::get_destination_path(&path, &manifest);
            let file_buffer = BufReader::new(File::open(&path).expect("cannot open file"));
            let tokens = Self::extract_template_tokens(file_buffer);
            let extract_tags = Self::extract_tags_from_tokens(&tokens);
            dbg!(&extract_tags);
        }

        return vec![String::new()];
    }

    fn load_paths_vec(manifest: Manifest) -> Vec<String> {
        let paths_file = manifest.template_paths_file.unwrap_or_default();
        let workdir = manifest.workdir.unwrap_or_default();
        let template_file_path = format!("{}/{}", workdir, paths_file);
        let paths_as_str = fs_extra::file::read_to_string(template_file_path).unwrap();
        let paths_vec: Vec<String> = serde_json::from_str(&paths_as_str).unwrap();
        return paths_vec;
    }

    fn extract_template_tokens(file_buffer: BufReader<File>) -> Vec<TemplateToken> {
        let result = file_buffer
            .lines()
            .filter_map(|l| l.ok())
            .enumerate()
            .filter_map(|(i, line)| Self::capture_token(i, line))
            .collect::<Vec<TemplateToken>>();
        return result;
    }

    fn capture_token(i: usize, line: String) -> Option<TemplateToken> {
        let caps = TOKEN_REGEX.captures(line.as_str());
        if let Some(c) = caps {
            let method = c.name("method").unwrap().as_str();
            return Some(TemplateToken {
                line_number: i,
                token: method.to_string(),
            });
        }
        return None;
    }

    fn get_destination_path(path: &String, manifest: &Manifest) -> PathBuf {
        let app_name = manifest.clone().app_name.unwrap_or_default();
        let workdir = manifest.clone().workdir.unwrap_or_default();
        let template_dir = manifest.clone().template_dir.unwrap_or_default();
        let to_strip = format!("{}/{}", &workdir, &template_dir);
        let template_path = Path::new(&path);
        let project_path = template_path
            .strip_prefix(to_strip)
            .expect("Error stripping prefix from path");
        let app_path = Path::new(&app_name);
        let dest = app_path.join(project_path);
        return dest;
    }

    fn extract_tags_from_tokens(tokens: &Vec<TemplateToken>) {}
}
