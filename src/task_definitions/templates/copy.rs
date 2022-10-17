use aho_corasick::AhoCorasick;
use std::{
    collections::HashMap,
    path::{Path, PathBuf}, fs,
};

use serde::{Deserialize, Serialize};

use crate::{logger::log_skip, lumberstack::Runnable, manifest::Manifest, system::System};

use super::{
    file_io::{TemplateFile, TemplateFileIO},
    tags::{should_task_run, TaskTag},
};

fn replacer(file_string: String, replace_vars: HashMap<String, String>) -> String {
    // dbg!(&replace_vars);
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
        let templates = TemplateFileIO::new(self.tag.to_owned(), self.manifest.clone());
        let file_tuples = Self::collect_file_tuples(&self.manifest, templates.unwrap());
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
    manifest: Manifest,
}

impl TemplateCopy {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<TemplateCopy> {
        if !should_task_run(&tag, &manifest) {
            log_skip(&tag.to_string());
            return None;
        }
        Some(TemplateCopy { tag, manifest })
    }

    fn collect_file_tuples(
        manifest: &Manifest,
        templates: Vec<TemplateFile>,
    ) -> Vec<(PathBuf, String)> {
        let replace_vars = manifest.replace.to_owned().unwrap_or_default();
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
