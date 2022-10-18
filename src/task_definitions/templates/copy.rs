use std::{
    fs::{File, self},
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;
use log::debug;
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::{app_config::AppConfig, file_io::FileIO, logger::log_skip, lumberstack::Runnable};

use super::{
    tags::{should_task_run, TaskTag},
    template_io::{TemplateFile, TemplateIO},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateCopy {
    tag: TaskTag,
    app_config: AppConfig,
}

impl TemplateCopy {
    pub fn new(tag: TaskTag, app_config: &AppConfig) -> Option<TemplateCopy> {
        if !should_task_run(&tag, &app_config) {
            log_skip(&tag.to_string());
            return None;
        }
        Some(TemplateCopy {
            tag,
            app_config: app_config.to_owned(),
        })
    }

    pub fn copy_template(template: TemplateFile) {
        debug!("Copying template: {}", &template.src.display());
        let src = &template.src;
        let dest = &template.dest;
        let exists = Path::exists(&dest);
        let is_dir = Path::is_dir(&src);
        let is_file = Path::is_file(&dest);

        if is_dir {
            debug!("is dir");
            let mut opts = CopyOptions::new();
            opts.overwrite = true;
            opts.skip_exist = false;
            opts.copy_inside = true;
            fs_extra::dir::copy(&src, &dest, &opts).unwrap();
            return;
        }

        if !exists && is_file {
            debug!("creating missing dir");
            Self::create_missing_dest_dir(&dest);
        }
        
        Self::write_template_to_dest(&template);
        debug!("template written");
    }

    fn write_template_to_dest(template: &TemplateFile) {
        let contents = &template.content;
        let src = &template.src;
        let dest = &template.dest;

        if let Some(contents) = contents {
            let mut file = File::create(&dest).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
            return;
        }
        // must be binary file at this point
        FileIO::copy(src, dest).unwrap();
    }

    fn create_missing_dest_dir(dest: &PathBuf) {
        let is_dir = Path::is_dir(&dest);
        let fileless_path = dest.parent().unwrap();
        if is_dir {
            fs::create_dir_all(&fileless_path).unwrap();
            return;
        }
        panic!("Cannot create parent directory for file: {:?}", dest);

    }
}

impl Runnable for TemplateCopy {
    fn run_job(&self) {
        let templates = TemplateIO::new(self.tag.to_owned(), &self.app_config);

        templates.unwrap().into_iter().for_each(|template| {
            Self::copy_template(template);
        });
    }
}
