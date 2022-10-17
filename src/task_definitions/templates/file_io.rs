use std::path::{Path, PathBuf};

use crate::{logger::log_skip, manifest::Manifest, system::System};
use ignore::{DirEntry, WalkBuilder};
use log::error;

use super::tags::{should_task_run, ReplaceVars, Symbol, Tags, TaskTag};

#[derive(Debug, Clone)]
pub struct TemplateFile {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub tags: Option<Tags>,
    pub replace_vars: Option<Vec<ReplaceVars>>,
    pub content: Option<String>,
}

pub struct TemplateFileIO;
impl TemplateFileIO {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<Vec<TemplateFile>> {
        if !should_task_run(&tag, &manifest) {
            log_skip(tag.to_string());
            return None;
        }

        let workdir = manifest.workdir.to_owned();
        let template_paths_file = manifest.template_paths_file.to_owned().unwrap_or_default();
        let full_templates_paths_file_path =
            format!("{}/{}", workdir.unwrap_or_default(), template_paths_file);
        let template_paths_file_str = System::file_as_str(full_templates_paths_file_path);
        if let None = template_paths_file_str {
            panic!(
                "[templateIO] Can not load paths file: {}",
                template_paths_file
            );
        }
        let from_file_template_paths = Self::read_paths_file(template_paths_file_str.unwrap());
        let from_dot_template_paths = Self::collect_dot_templates(&from_file_template_paths);
        let combined_templates = [
            from_file_template_paths.clone(),
            from_dot_template_paths.clone(),
        ]
        .concat();
        let all_templates = Self::process_combined_templates(&manifest, combined_templates);
        Some(all_templates)
    }

    fn process_combined_templates(
        manifest: &Manifest,
        templates: Vec<PathBuf>,
    ) -> Vec<TemplateFile> {
        let tags_to_run = manifest.tags.to_owned().unwrap_or_default();
        let tags_to_skip = manifest.skip_tags.to_owned().unwrap_or_default();
        let dest_root_dir = manifest.app_name.to_owned().unwrap_or_default();
        let to_strip = manifest.full_template_path.to_owned().unwrap_or_default();

        let replaced_symbol = |pathbuf: &PathBuf| {
            let src = pathbuf.to_owned();
            let dest = Self::strip_from_path(&dest_root_dir, &to_strip, pathbuf.as_path());
            let (tags, replace_vars) = Symbol::new(pathbuf.to_owned());
            return (src, dest, tags, replace_vars);
        };

        let transformed = templates
            .iter()
            .map(replaced_symbol)
            .filter(|(_, _, own_tags, _)| Self::should_run_tag(own_tags.to_owned(), manifest))
            .map(|(src, dest, tags, replace_vars)| {
                return TemplateFile {
                    src,
                    dest,
                    tags,
                    replace_vars,
                    content: None,
                };
            })
            .collect::<Vec<TemplateFile>>();
        return transformed;
    }

    fn should_run_tag(own_tags: Option<Vec<String>>, manifest: &Manifest) -> bool {
        let tags_to_run = manifest.tags.to_owned().unwrap_or_default();
        let tags_to_skip = manifest.skip_tags.to_owned().unwrap_or_default();
        if let Some(ot) = own_tags {
            let in_tags_to_skip = ot
                .iter()
                .filter(|t| tags_to_skip.contains(t))
                .collect::<Vec<_>>();

            let in_tags_to_run = ot
                .iter()
                .filter(|t| tags_to_run.contains(t))
                .collect::<Vec<_>>();

            if in_tags_to_run.len().gt(&0) || !in_tags_to_skip.len().gt(&0) {
                return true;
            }

            log_skip(format!("{:?}", in_tags_to_run));
            return false;
        }
        return true;
    }

    fn collect_dot_templates(all_templates: &Vec<PathBuf>) -> Vec<PathBuf> {
        all_templates
            .iter()
            .filter(|f| f.ends_with(".template"))
            .map(|f| {
                let mut buf = f.clone();
                buf.pop();
                return buf;
            })
            .flat_map(|pathbuf| Self::collect_dir(pathbuf))
            .filter(|f| f.path().is_file())
            .map(|de| de.into_path())
            .collect::<Vec<PathBuf>>()
    }

    fn collect_dir<P: AsRef<Path>>(dir: P) -> Vec<DirEntry> {
        WalkBuilder::new(dir)
            .standard_filters(false)
            .add_custom_ignore_filename(".templateignore")
            .build()
            .filter_map(|f| f.ok())
            .collect::<Vec<DirEntry>>()
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

    fn read_paths_file(json: String) -> Vec<PathBuf> {
        let loaded: Vec<String> = serde_json::from_str(&json.as_str())
            .map_err(|_| Self::generate_read_error("to json struct".to_string()))
            .unwrap();

        loaded
            .iter()
            .map(|i| PathBuf::from(i))
            .collect::<Vec<PathBuf>>()
    }

    fn generate_read_error(path: String) {
        let msg = format!("[templates] Error reading file {}", path);
        error!("{}", msg);
    }
}

struct TemplatePaths;

impl TemplatePaths {
    fn read_paths_file(json: String) -> Vec<PathBuf> {
        let loaded: Vec<String> = serde_json::from_str(&json.as_str())
            .map_err(|_| Self::generate_read_error("to json struct".to_string()))
            .unwrap();

        loaded
            .iter()
            .map(|i| PathBuf::from(i))
            .collect::<Vec<PathBuf>>()
    }

    fn generate_read_error(path: String) {
        let msg = format!("[templates] Error reading file {}", path);
        error!("{}", msg);
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use super::*;

//     #[test]
//     fn it_tests() {
//         let template_tags = Some(vec!["foo".to_string()]);
//         let mock_src = PathBuf::from("tostrip/file.js");
//         let mock_dest = PathBuf::from("myapp/file.js");

//         let template = TemplateFile { src: mock_src, dest: mock_dest, tags: template_tags, replace_vars: None };

//         let mock_file = PathBuf::from("tostrip/foo.txt");

//         let templates = vec![mock_file];

//         let manifest = &Manifest {
//             tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
//             full_template_path: Some(String::from("tostrip")),
//             ..Manifest::default()
//         };

//         let actual = TemplateFileIO::process_combined_templates(manifest, templates);

//         dbg!(actual);
//     }
// }
