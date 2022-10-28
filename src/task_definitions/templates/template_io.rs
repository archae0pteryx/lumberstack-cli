use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::{
    app_config::AppConfig, system::file_io::FileIO, system::logger::log_task_skip,
    task_definitions::templates::symbols::Symbols,
};
use ignore::{DirEntry, WalkBuilder};
use log::{debug, error};

use super::{
    replace_vars::Replacer,
    tags::{should_task_run, TaskTag},
};

#[derive(Debug, Clone)]
pub struct TemplateFile {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub tags: Vec<String>,
    pub content: Option<String>,
}

pub struct TemplateIO;
impl TemplateIO {
    pub fn processed_templates(
        tag: TaskTag,
        app_config: &AppConfig,
    ) -> Result<Option<Vec<TemplateFile>>> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag.to_string());
            return Ok(None);
        }
        let all_paths = Self::gather_all_template_paths(&app_config.template_map)?;
        let processed_templates = Self::process_all_templates(app_config, all_paths);

        assert!(!processed_templates.is_empty());
        Ok(Some(processed_templates))
    }

    pub fn gather_all_template_paths(map_file: &str) -> Result<Vec<PathBuf>> {
        let f = FileIO::read_or_fail(&map_file)?;
        let parent_paths = Self::deserialize_paths_map(f);
        let child_paths = Self::collect_dot_templates(&parent_paths);
        let all_template_paths = [parent_paths, child_paths].concat();
        Ok(all_template_paths)
    }

    // Core template processing
    fn process_all_templates(
        app_config: &AppConfig,
        template_paths: Vec<PathBuf>,
    ) -> Vec<TemplateFile> {
        let dest_root_dir = &app_config.app_name;
        let path_to_strip = &app_config.template_dir;

        let transformed = template_paths
            .iter()
            .map(|template_path| {
                let src = template_path.to_owned();
                let dest =
                    Self::strip_from_path(dest_root_dir, path_to_strip, template_path.as_path());
                if FileIO::is_not_contentful(&src) {
                    debug!("Template is not text: {}", src.to_str().unwrap());
                    return TemplateFile {
                        src,
                        dest,
                        tags: vec![],
                        content: None,
                    };
                }

                let file_str = FileIO::read_or_skip(&src).expect("Error reading file");

                let tags = Symbols::get_tags(&file_str);

                let replaced_content =
                    Replacer::process_and_replace_vars(&file_str, app_config.clone());
                let sanitized_content = Symbols::remove_symbols(&replaced_content);
                TemplateFile {
                    src,
                    dest,
                    tags,
                    content: Some(sanitized_content),
                }
            })
            .collect::<Vec<_>>();
        transformed
    }

    fn collect_dot_templates(all_templates: &[PathBuf]) -> Vec<PathBuf> {
        all_templates
            .iter()
            .filter(|f| f.ends_with(".template"))
            .map(|f| {
                let mut buf = f.clone();
                buf.pop();
                buf
            })
            .flat_map(Self::collect_dir)
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

        app_path.join(stripped)
    }

    fn deserialize_paths_map(json: String) -> Vec<PathBuf> {
        let loaded: Vec<String> = serde_json::from_str(json.as_str())
            .map_err(|_| Self::generate_read_error("to json struct".to_string()))
            .unwrap();

        loaded.iter().map(PathBuf::from).collect::<Vec<PathBuf>>()
    }

    fn generate_read_error(path: String) {
        let msg = format!("[templates] Error reading file {}", path);
        error!("{}", msg);
    }
}
