#![allow(unused)]

use self::{auth::authenticate_cli, template::copy_heroku_templates};
use crate::cli::arguments::{has_only_enabled, is_enabled, CliOptions};
use crate::cli::{app_paths::AppPaths, progress::AppProgress};
use crate::system::error::AppError;
use crate::system::shell::Shell;

mod auth;
mod template;

pub struct Heroku;

impl Heroku {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        if is_enabled(&CliOptions::Heroku) || !has_only_enabled() {}
        Ok(())
    }

    fn install_dependencies() -> Result<(), AppError> {
        Shell::exec(format!("cd {}; yarn add pm2", AppPaths::root(None)))?;
        Ok(())
    }
}
