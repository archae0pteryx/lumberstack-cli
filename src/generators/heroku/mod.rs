#![allow(unused)]

use self::{auth::authenticate_cli, template::copy_heroku_templates};
use crate::cli::arguments::{CliOptions, is_enabled, has_only_enabled};
use crate::cli::{app_paths::AppPaths, logger::Logger, progress::AppProgress};
use crate::system::error::AppError;
use crate::system::shell::Shell;
use question::{Answer, Question};

mod auth;
mod template;

pub struct Heroku;

impl Heroku {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        if is_enabled(&CliOptions::Heroku) || !has_only_enabled() {
            // if Self::confirm_setup() {
            //     progress_bar.update("🧱 Setting up Heroku");
            //     Self::install_dependencies()?;
            //     copy_heroku_templates(progress_bar)?;
            //     authenticate_cli()?;
            // } else {
            //     Logger::loud_info(String::from("Skipping heroku setup"));
            // }
        }
        Ok(())
    }

    fn confirm_setup() -> bool {
        let should_setup_heroku = Question::new("Would you like to setup heroku?").confirm();
        should_setup_heroku == Answer::YES
    }

    fn install_dependencies() -> Result<(), AppError> {
        Shell::exec(format!("cd {}; yarn add pm2", AppPaths::root(None)))?;
        Ok(())
    }
}
