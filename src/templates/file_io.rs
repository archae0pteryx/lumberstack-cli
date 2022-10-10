use std::{
    path::{Path, PathBuf},
};

use crate::{
    ansible::playbook::create::Playbook,
    manifest::Manifest,
    system::System,
    tags::{should_task_run, ReplaceVars, Symbol, TaskTag},
};
use ignore::{DirEntry, WalkBuilder};
use log::error;

#[derive(Debug, Clone)]
pub struct TemplateFile {
    src: PathBuf,
    dest: PathBuf,
    tags: Option<Vec<String>>,
    replace_vars: Option<Vec<ReplaceVars>>,
}

pub struct TemplateFileIO;
impl TemplateFileIO {
    pub fn new(tag: TaskTag, manifest: Manifest) -> anyhow::Result<Option<Playbook>> {
        let tags = manifest.tags.to_owned();
        if !should_task_run(&tag, &tags) {
            return Ok(None);
        }
        let template_paths_file = manifest.template_paths_file.to_owned().unwrap_or_default();
        let template_paths_file_str = System::load_file(PathBuf::from(template_paths_file))?;
        let from_file_template_paths = Self::read_paths_file(template_paths_file_str);
        let from_dot_template_paths = Self::collect_dot_templates(&from_file_template_paths);
        let combined_templates = [
            from_file_template_paths.clone(),
            from_dot_template_paths.clone(),
        ]
        .concat();
        let all_templates = Self::process_combined_templates(&manifest, combined_templates);

        // dbg!(&all_templates);
        Ok(Some(Playbook::new("Template Parser")))
    }

    fn process_combined_templates(
        manifest: &Manifest,
        templates: Vec<PathBuf>,
    ) -> Vec<TemplateFile> {
        let dest_root_dir = manifest.app_name.to_owned().unwrap_or_default();
        let to_strip = manifest.full_template_path.to_owned().unwrap_or_default();
        let transformed = templates
            .iter()
            .map(|pathbuf| {
                let dest = Self::strip_from_path(&dest_root_dir, &to_strip, pathbuf.as_path());
                let (tags, replace_vars) = Symbol::new(pathbuf.to_owned());
                return TemplateFile {
                    src: pathbuf.to_owned(),
                    dest: dest.to_owned(),
                    tags,
                    replace_vars,
                };
            })
            .collect::<Vec<TemplateFile>>();
        return transformed;
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
