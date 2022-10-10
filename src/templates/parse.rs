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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_gathers_template_files() {
//         assert!(true);
//     }
// }
