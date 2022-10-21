use std::path::Path;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    app_config::AppConfig, lumberstack::Runnable, spinner::create_spinner, system::file_io::FileIO,
    system::logger::log_task_skip,
};

use super::{
    tags::{should_task_run, TaskTag},
    template_io::{TemplateFile, TemplateIO},
};

impl Runnable for TemplateCopy {
    fn run_job(&self) {
        let spinner = create_spinner("Copying templates");
        spinner.set_prefix("💾");
        let templates = TemplateIO::processed_templates(self.tag.to_owned(), &self.app_config);

        templates.unwrap().into_iter().for_each(|template| {
            Self::copy_template(template);
        });
        debug!("Finished tag: {}", &self.tag.to_string());
        spinner.finish_and_clear();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateCopy {
    tag: TaskTag,
    app_config: AppConfig,
}

impl TemplateCopy {
    pub fn inject_templates(tag: TaskTag, app_config: &AppConfig) -> Option<TemplateCopy> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(&tag.to_string());
            return None;
        }
        Some(TemplateCopy {
            tag,
            app_config: app_config.to_owned(),
        })
    }

    pub fn copy_template(template: TemplateFile) {
        let src = &template.src;
        let dest = &template.dest;
        let dest_dir = &dest.parent().unwrap();
        let dest_dir_exists = &dest.exists();

        if !dest_dir_exists {
            FileIO::create_dir(dest_dir).unwrap();
        }

        if Path::is_dir(&template.src) {
            return FileIO::copy_dir(src, dest);
        }

        Self::write_template_to_dest(&template);
    }

    fn write_template_to_dest(template: &TemplateFile) {
        let contents = &template.content;
        let src = &template.src;
        let dest = &template.dest;

        if let Some(contents) = contents {
            FileIO::write(dest, contents).unwrap();
            return;
        }
        // must be binary file at this point
        debug!("Copying binary file: {}", &src.display());
        FileIO::copy(src, dest).unwrap();
    }
}
