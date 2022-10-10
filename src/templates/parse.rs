use std::{
    path::{Path, PathBuf},
};

use crate::{
    ansible::playbook::create::Playbook,
    manifest::{Manifest},
    tags::{should_task_run, TaskTag}, TEMPLATE_TOKEN_REGEX,
};
use ignore::{DirEntry, WalkBuilder};
use lazy_static::lazy_static;
use regex::Regex;

pub struct TemplateParser;

#[derive(Debug, Clone)]
pub struct TemplateFile {
    path: PathBuf,
    tags: Option<Vec<String>>,
}

impl TemplateParser {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<Playbook> {
        let tags = manifest.tags.to_owned();
        if !should_task_run(&tag, &tags) {
            return None;
        }
        let single_templates = Self::collect_single_templates(&manifest);
        let dot_template_files = Self::collect_dot_templates(&single_templates);
        let all_template_paths = [single_templates.clone(), dot_template_files.clone()].concat();
        let processed_templates = Self::process_combined_templates(&manifest, all_template_paths);

        dbg!(&processed_templates);
        Some(Playbook::new("Template Parser"))
    }

    fn collect_single_templates(manifest: &Manifest) -> Vec<DirEntry> {
        let cloned_templates_dir = manifest.full_template_path.to_owned().unwrap_or_default();
        WalkBuilder::new(cloned_templates_dir)
            .standard_filters(false)
            .add_custom_ignore_filename(".templateignore")
            .build()
            .filter_map(|f| f.ok())
            .filter(|f| {
                let file = fs_extra::file::read_to_string(f.path());
                if let Ok(contents) = file {
                    return Self::is_template_file(&contents);
                }
                return false;
            })
            .collect::<Vec<DirEntry>>()
    }

    fn process_combined_templates(
        manifest: &Manifest,
        templates: Vec<DirEntry>,
    ) -> Vec<TemplateFile> {
        let dest_root_dir = manifest.app_name.to_owned().unwrap_or_default();
        let to_strip = manifest.full_template_path.to_owned().unwrap_or_default();
        let transformed = templates
            .iter()
            .map(|tf| {
                let src_to_dest_path = Self::strip_from_path(&dest_root_dir, &to_strip, tf.path());
                let extracted_tags = Self::extract_tags(tf.path());
                return TemplateFile {
                    path: src_to_dest_path,
                    tags: extracted_tags,
                };
            })
            .collect::<Vec<TemplateFile>>();
        return transformed;
    }

    fn collect_dot_templates(all_templates: &Vec<DirEntry>) -> Vec<DirEntry> {
        all_templates
            .iter()
            .filter(|f| f.file_name().eq(".template"))
            .map(|f| {
                let mut buf = PathBuf::from(f.path());
                buf.pop();
                return buf;
            })
            .flat_map(|pathbuf| Self::collect_dir(pathbuf))
            .filter(|f| f.path().is_file())
            .collect::<Vec<DirEntry>>()
    }

    fn collect_dir<P: AsRef<Path>>(dir: P) -> Vec<DirEntry> {
        WalkBuilder::new(dir)
            .standard_filters(false)
            .add_custom_ignore_filename(".templateignore")
            .build()
            .filter_map(|f| f.ok())
            .collect::<Vec<DirEntry>>()
    }

    fn is_template_file(file_str: &String) -> bool {
        // lazy static to not recompile regex over and over
        lazy_static! {
            static ref TEMPLATE_RE: Regex = Regex::new(TEMPLATE_TOKEN_REGEX).unwrap();
        }

        if let Some(_) = TEMPLATE_RE.find(file_str) {
            return true;
        }
        return false;
    }

    fn strip_from_path<P: AsRef<Path>>(
        dest_root_dir: &String,
        to_strip: &String,
        file: P,
    ) -> PathBuf {
        let stripped = file
            .as_ref()
            .strip_prefix(to_strip)
            .expect("Error stripping prefix from path");
        let app_path = Path::new(&dest_root_dir);
        let dest = app_path.join(stripped);
        return dest;
    }

    fn extract_tags<P: AsRef<Path>>(template_file: P) -> Option<Vec<String>> {

        None
    }
}


pub mod clone;
// mod copy;
// mod parse;


// use crate::manifest::Manifest;
// use crate::TEMPLATE_TOKEN_REGEX;
// use indicatif::ProgressBar;
// use lazy_static::lazy_static;
// use regex::Regex;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::path::Path;
// use std::path::PathBuf;

// #[derive(Debug)]
// pub struct TemplateFile {
//     pub src: PathBuf,
//     pub dest: PathBuf,
//     pub symbols: Vec<TemplateSymbol>,
// }

// #[derive(Debug)]
// pub struct TemplateSymbol {
//     pub line_number: usize,
//     pub tags: Option<Vec<String>>,
//     pub replace_vars: Option<Vec<String>>,
// }

// lazy_static! {
//     static ref TOKEN_REGEX: Regex = Regex::new(TEMPLATE_TOKEN_REGEX).unwrap();
//     static ref TOKEN_METHOD_REGEX: Regex = Regex::new(r#"tags\((?P<tags>.*)\)"#).unwrap();
//     static ref REPLACE_METHOD_REGEX: Regex = Regex::new(r#"replace.*\((?P<replace>.*)\)"#).unwrap();
// }

// pub struct Templates;

// impl Templates {
//     pub fn collect_templates(manifest: Manifest, spinner: &ProgressBar) -> Vec<TemplateFile> {
//         spinner.set_message("Collecting templates...");
//         let paths = Self::load_paths_vec(manifest.clone());
//         let mut template_files: Vec<TemplateFile> = Vec::new();
//         for path in paths {
//             let dest_path = Self::get_destination_path(&path, &manifest);
//             let file_buffer = BufReader::new(File::open(&path).expect("cannot open file"));
//             let template_symbols = Self::gather_symbols_for_file(file_buffer);

//             let file = TemplateFile {
//                 src: PathBuf::from(path),
//                 dest: dest_path,
//                 symbols: template_symbols,
//             };

//             template_files.push(file);
//         }
//         // dbg!(&template_files);
//         return template_files;
//     }

//     fn load_paths_vec(manifest: Manifest) -> Vec<String> {
//         let paths_file = manifest.template_paths_file.unwrap_or_default();
//         let workdir = manifest.workdir.unwrap_or_default();
//         let template_file_path = format!("{}/{}", workdir, paths_file);
//         let paths_as_str = fs_extra::file::read_to_string(template_file_path).unwrap();
//         let paths_vec: Vec<String> = serde_json::from_str(&paths_as_str).unwrap();
//         return paths_vec;
//     }

//     fn gather_symbols_for_file(file_buffer: BufReader<File>) -> Vec<TemplateSymbol> {
//         let result = file_buffer
//             .lines()
//             .filter_map(|l| l.ok())
//             .enumerate()
//             .filter_map(|(i, line)| Self::capture_token_method(i, line))
//             .map(|(line_number, method)| {
//                 let tags = Self::create_tag_vec(&method);
//                 let replace_vars = Self::create_replace_vec(&method);
//                 return TemplateSymbol {
//                     line_number,
//                     tags,
//                     replace_vars,
//                 };
//             })
//             .collect::<Vec<TemplateSymbol>>();
//         return result;
//     }

//     fn capture_token_method(i: usize, line: String) -> Option<(usize, String)> {
//         let caps = TOKEN_REGEX.captures(line.as_str());
//         if let Some(c) = caps {
//             return Some((i, c.name("method").unwrap().as_str().to_string()));
//         }
//         return None;
//     }

//     fn get_destination_path(path: &String, manifest: &Manifest) -> PathBuf {
//         let app_name = manifest.clone().app_name.unwrap_or_default();
//         let workdir = manifest.clone().workdir.unwrap_or_default();
//         let template_dir = manifest.clone().template_dir.unwrap_or_default();
//         let to_strip = format!("{}/{}", &workdir, &template_dir);
//         let template_path = Path::new(&path);
//         let project_path = template_path
//             .strip_prefix(to_strip)
//             .expect("Error stripping prefix from path");
//         let app_path = Path::new(&app_name);
//         let dest = app_path.join(project_path);
//         return dest;
//     }

//     fn create_tag_vec(method: &String) -> Option<Vec<String>> {
//         let str_tags = Self::extract_tags(&method);
//         if let Some(str_tag) = str_tags {
//             return Some(
//                 str_tag
//                     .split(",")
//                     .map(String::from)
//                     .collect::<Vec<String>>(),
//             );
//         }
//         return None;
//     }

//     fn create_replace_vec(method: &String) -> Option<Vec<String>> {
//         let str_vars = Self::extract_vars(method);
//         if let Some(vars) = str_vars {
//             return Some(vars.split(",").map(String::from).collect::<Vec<String>>());
//         }
//         return None;
//     }

//     fn extract_tags(method: &String) -> Option<String> {
//         let captured_tags = TOKEN_METHOD_REGEX.captures(method);
//         if let Some(c) = captured_tags {
//             return Some(c.name("tags").unwrap().as_str().to_string());
//         }
//         return None;
//     }

//     fn extract_vars(method: &String) -> Option<String> {
//         let captured_vars = REPLACE_METHOD_REGEX.captures(method);
//         if let Some(c) = captured_vars {
//             return Some(c.name("replace").unwrap().as_str().to_string());
//         }
//         return None;
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_gathers_template_files() {
//         assert!(true);
//     }
// }
