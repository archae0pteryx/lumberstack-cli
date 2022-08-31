use self::checks::{check_app_installed, check_node_version, os_ok, check_docker};
use crate::cli::progress::AppProgress;
use std::io::Result;

mod checks;
pub mod error;
pub mod shell;
pub mod utils;
pub struct System;

impl System {
    pub fn init(progress_bar: &AppProgress) -> Result<()> {
        progress_bar.update("🔦 Checking system requirements");
        os_ok();
        check_app_installed("yarn")?;
        check_docker()?;
        check_app_installed("heroku")?;
        check_app_installed("node")?;
        check_node_version()?;
        Ok(())
    }
}
