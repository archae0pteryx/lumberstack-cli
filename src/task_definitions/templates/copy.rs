use aho_corasick::AhoCorasick;
use std::{
    collections::HashMap,
    path::{Path, PathBuf}
};

use serde::{Deserialize, Serialize};

use crate::{logger::log_skip, lumberstack::Runnable, system::System, app_config::AppConfig};

use super::{
    file_io::{TemplateFile, TemplateFileIO},
    tags::{should_task_run, TaskTag},
};

fn replacer(file_string: String, replace_vars: HashMap<String, String>) -> String {
    let keys = replace_vars
        .clone()
        .into_iter()
        .map(|(key, _)| key)
        .collect::<Vec<String>>();
    let values = replace_vars
        .clone()
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<String>>();

    if keys.len().ne(&values.len()) {
        panic!("Error parsing replacer vars!");
    }

    let ac = AhoCorasick::new(keys);
    let result = ac.replace_all(&file_string, &values);
    return result;
}

impl Runnable for TemplateCopy {
    fn run_job(&self) {
        let templates = TemplateFileIO::new(self.tag.to_owned(), &self.app_config);
        let file_tuples = Self::collect_file_tuples(&self.app_config, templates.unwrap());
        // dbg!(&file_tuples);
        file_tuples.into_iter().for_each(|(dest, file_str)| {
            // let is_image = System::is_image(dest);
            // if is_image {
            //     fs::copy(from, to)
            // }

            fs_extra::file::write_all(&dest, &file_str).expect("Error writing template");
            // debug!("wrote: {}", &dest.display());
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateCopy {
    tag: TaskTag,
    app_config: AppConfig,
}

impl TemplateCopy {
    pub fn new(tag: TaskTag, app_config: AppConfig) -> Option<TemplateCopy> {
        if !should_task_run(&tag, &app_config) {
            log_skip(&tag.to_string());
            return None;
        }
        Some(TemplateCopy { tag, app_config })
    }

    fn collect_file_tuples(
        app_config: &AppConfig,
        templates: Vec<TemplateFile>,
    ) -> Vec<(PathBuf, String)> {
        let replace_vars = app_config.template_vars.to_owned();
        templates
            .iter()
            .filter(|t| !Path::new(&t.src).ends_with(".template"))
            .map(|t| (t, System::file_as_str(&t.src)))
            .filter(|(_, fs)| fs.is_some())
            .map(|(t, fs)| {
                (
                    t.dest.to_owned(),
                    replacer(fs.unwrap(), replace_vars.to_owned()),
                )
            })
            .inspect(|f| {
                dbg!(&f.0);
            })
            .collect::<Vec<(PathBuf, String)>>()
    }
}
