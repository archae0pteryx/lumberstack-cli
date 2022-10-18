use std::path::{Path, PathBuf};

use crate::{app_config::AppConfig, system::file_io::FileIO, system::logger::log_skip};
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

pub struct TemplateIO;
impl TemplateIO {
    pub fn new(tag: TaskTag, app_config: &AppConfig) -> Option<Vec<TemplateFile>> {
        if !should_task_run(&tag, &app_config) {
            log_skip(tag.to_string());
            return None;
        }

        let template_map_file = app_config.template_map.to_owned();
        let template_map_file_str = FileIO::read(&app_config.template_map);
        if let None = template_map_file_str {
            panic!(
                "[templateIO] Can not load paths file: {}",
                template_map_file
            );
        }
        let from_file_template_paths = Self::read_paths_file(template_map_file_str.unwrap());
        let from_dot_template_paths = Self::collect_dot_templates(&from_file_template_paths);
        let combined_templates = [
            from_file_template_paths.clone(),
            from_dot_template_paths.clone(),
        ]
        .concat();
        let processed_templates = Self::process_combined_templates(&app_config, combined_templates);
        Some(processed_templates)
    }

    fn process_combined_templates(
        app_config: &AppConfig,
        templates: Vec<PathBuf>,
    ) -> Vec<TemplateFile> {
        let dest_root_dir = app_config.app_name.to_owned();
        let to_strip = app_config.template_dir.to_owned();

        let prep_paths_tags_vars = |pathbuf: &PathBuf| {
            let src = pathbuf.to_owned();
            let dest = Self::strip_from_path(&dest_root_dir, &to_strip, pathbuf.as_path());
            let (tags, replace_vars) = Symbol::new(pathbuf.to_owned());
            return (src, dest, tags, replace_vars);
        };

        let transformed = templates
            .iter()
            .map(prep_paths_tags_vars)
            .filter(|(_, _, own_tags, _)| Self::should_run_tag(own_tags.to_owned(), app_config))
            .map(|template_data| Self::process_template_content(&template_data))
            .collect::<Vec<TemplateFile>>();
        return transformed;
    }

    fn process_template_content(
        template_data: &(PathBuf, PathBuf, Option<Tags>, Option<Vec<ReplaceVars>>),
    ) -> TemplateFile {
        let src = &template_data.0;
        let dest = &template_data.1;
        let tags = &template_data.2;
        let replace_vars = &template_data.3;

        if FileIO::is_image(&src) {
            return TemplateFile {
                src: src.to_owned(),
                dest: dest.to_owned(),
                tags: tags.to_owned(),
                replace_vars: None,
                content: None,
            };
        }

        let mut content = FileIO::read(&src).unwrap();

        if let Some(vars) = replace_vars {
            for obj in vars {
                content = content.replace(&obj.key, &obj.value);
            }
        }

        TemplateFile {
            src: src.to_owned(),
            dest: dest.to_owned(),
            tags: tags.to_owned(),
            replace_vars: replace_vars.to_owned(),
            content: Some(content),
        }
    }

    fn should_run_tag(own_tags: Option<Vec<String>>, app_config: &AppConfig) -> bool {
        let tags_to_run = app_config.tags.to_owned().unwrap_or_default();
        let tags_to_skip = app_config.skip_tags.to_owned().unwrap_or_default();
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
