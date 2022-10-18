use std::path::Path;

use log::debug;
use serde::{Deserialize, Serialize};

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
        let dest = &template.dest;
        let dest_dir = &dest.parent().unwrap();
        let dest_dir_exists = &dest.exists();

        if !dest_dir_exists {
            FileIO::create_dir(dest_dir).unwrap();
        }

        Self::write_template_to_dest(&template);
        debug!("template written");
    }

    fn write_template_to_dest(template: &TemplateFile) {
        let contents = &template.content;
        let src = &template.src;
        let dest = &template.dest;

        if Path::is_dir(&template.src) {
            FileIO::copy_dir(&src, &dest);
            return;
        }

        if let Some(contents) = contents {
            FileIO::write(&dest, contents).unwrap();
            return;
        }
        // must be binary file at this point
        FileIO::copy(src, dest).unwrap();
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
