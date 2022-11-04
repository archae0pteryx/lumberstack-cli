use self::{checks::SystemChecks, logger::Logger};
use crate::app_config::{load_app_config, AppConfig};
use anyhow::Result;

pub mod checks;
pub mod cli_args;
pub mod commands;
pub mod config_file;
pub mod file_io;
pub mod logger;
pub mod spinner;

pub struct System;

impl System {
    pub fn init() -> Result<AppConfig> {
        let app_config = load_app_config()?;
        Logger::init(&app_config);
        SystemChecks::init(&app_config)?;
        Ok(app_config)
    }
}
