mod cmds;
mod template;

use self::{
    cmds::{down_db, up_db},
    template::create_compose,
};
use crate::{cli::progress::AppProgress, system::error::AppError};

pub struct Docker;

impl Docker {
    pub fn start_db(database: &str, progress_bar: &AppProgress) -> Result<(), AppError> {
        down_db(database, progress_bar).ok();
        up_db(database, progress_bar)?;
        Ok(())
    }

    pub fn copy_compose(progress_bar: &AppProgress) -> Result<(), AppError> {
        create_compose(progress_bar)?;
        Ok(())
    }

    pub fn cleanup(progress_bar: &AppProgress) -> Result<(), AppError> {
        progress_bar.update("🐋 Cleaning up docker");
        down_db("db", progress_bar).ok();
        down_db("testdb", progress_bar).ok();
        Ok(())
    }
}
