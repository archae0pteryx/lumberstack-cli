use self::{env::*, github::*, markdown::*, scripts::*};
use crate::{
    cli::{
        arguments::{is_enabled, CliOptions, has_only_enabled},
        progress::*,
    },
    system::{error::AppError, utils::*},
};
use serde_json::Value;

use super::docker::Docker;

mod env;
mod github;
mod markdown;
mod scripts;

pub struct FileMap {
    pub source: String,
    pub destination: String,
    pub replace_map: Option<Value>,
}

pub struct Templates;

impl Templates {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        if is_enabled(&CliOptions::Templates) || !has_only_enabled() {
            progress_bar.update("🚛 Loading templates");
            AppEnv::copy_template()?;
            Docker::copy_compose(progress_bar)?;
            Markdown::copy_templates()?;
            Scripts::copy_template()?;
            GithubActions::copy_template()?;
        }
        Ok(())
    }

    pub fn replace_and_load(templates: Vec<FileMap>) -> Result<(), AppError> {
        for mapping in templates.iter() {
            let template =
                HandlebarBuilder::new(mapping.source.to_string(), mapping.destination.to_string());
            template.create(mapping.replace_map.clone())?;
        }
        Ok(())
    }
}
