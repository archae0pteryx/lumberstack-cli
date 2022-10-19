use std::path::{Path, PathBuf};

use crate::{app_config::AppConfig, system::file_io::FileIO, system::logger::log_task_skip};
use ignore::{DirEntry, WalkBuilder};
use log::{debug, error};

use super::{
    replace_vars::Replacer,
    tags::{should_task_run, Symbols, TaskTag},
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
    pub fn new(tag: TaskTag, app_config: &AppConfig) -> Option<Vec<TemplateFile>> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag.to_string());
            return None;
        }

        let template_map_file = app_config.template_map.to_owned();
        let template_map_file_str = FileIO::read(&app_config.template_map);
        if template_map_file_str.is_none() {
            panic!(
                "[templateIO] Can not load paths file: {}",
                template_map_file
            );
        }
        let from_file_template_paths = Self::read_paths_file(template_map_file_str.unwrap());
        let from_dot_template_paths = Self::collect_dot_templates(&from_file_template_paths);
        let combined_templates = [
            from_file_template_paths,
            from_dot_template_paths,
        ]
        .concat();
        let processed_templates = Self::process_all_templates(app_config, combined_templates);
        assert!(!processed_templates.is_empty());

        Some(processed_templates)
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
                    debug!("Template is not contentful: {}", src.to_str().unwrap());
                    return TemplateFile {
                        src,
                        dest,
                        tags: vec![],
                        content: None,
                    };
                }

                let file_str = FileIO::read(&src).expect("Error reading file");

                let tags = Symbols::parse_tags(&file_str);

                let replaced_content =
                    Replacer::process_and_replace_vars(&file_str, app_config.clone());

                TemplateFile {
                    src,
                    dest,
                    tags,
                    content: Some(replaced_content),
                }
            })
            .collect::<Vec<_>>();
        transformed
    }

    // fn process_template_content(mut template_file: TemplateFile) -> TemplateFile {
    //     let src = template_file.src;
    //     let symbols = template_file.symbols;
    //     let replace_vars = symbols.symbol_replace_vars;
    //     if FileIO::is_image(&template_file.src) {
    //         return template_file;
    //     }

    //     let mut content = FileIO::read(&src).unwrap();

    //     if let Some(vars) = replace_vars {
    //         for obj in vars {
    //             dbg!(&obj);
    //             content = content.replace(&obj.key, &obj.value);
    //         }
    //     }

    //     TemplateFile {
    //         src: src.to_owned(),
    //         dest: dest.to_owned(),
    //         symbol_tags: tags.to_owned(),
    //         symbol_replace_vars: replace_vars.to_owned(),
    //         content: Some(content),
    //     }
    // }

    // fn should_run_tag(own_tags: Option<Vec<String>>, app_config: &AppConfig) -> bool {
    //     let tags_to_run = app_config.tags.to_owned().unwrap_or_default();
    //     let tags_to_skip = app_config.skip_tags.to_owned().unwrap_or_default();
    //     if let Some(ot) = own_tags {
    //         let in_tags_to_skip = ot
    //             .iter()
    //             .filter(|t| tags_to_skip.contains(t))
    //             .collect::<Vec<_>>();

    //         let in_tags_to_run = ot
    //             .iter()
    //             .filter(|t| tags_to_run.contains(t))
    //             .collect::<Vec<_>>();

    //         if in_tags_to_run.len().gt(&0) || !in_tags_to_skip.len().gt(&0) {
    //             return true;
    //         }

    //         log_task_skip(format!("{:?}", in_tags_to_run));
    //         return false;
    //     }
    //     return true;
    // }

    fn collect_dot_templates(all_templates: &Vec<PathBuf>) -> Vec<PathBuf> {
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

    fn read_paths_file(json: String) -> Vec<PathBuf> {
        let loaded: Vec<String> = serde_json::from_str(json.as_str())
            .map_err(|_| Self::generate_read_error("to json struct".to_string()))
            .unwrap();

        loaded
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>()
    }

    fn generate_read_error(path: String) {
        let msg = format!("[templates] Error reading file {}", path);
        error!("{}", msg);
    }
}
