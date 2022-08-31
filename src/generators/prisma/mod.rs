use self::cmds::migrate;
use self::templates::{copy_seed, create_schema};

use crate::cli::arguments::{is_enabled, CliOptions, has_only_enabled};
use crate::cli::progress::AppProgress;
use crate::system::error::AppError;

use super::docker::Docker;

mod cmds;
mod templates;
pub struct Prisma;

impl Prisma {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        let enabled = is_enabled(&CliOptions::Prisma);
        let has_only = has_only_enabled();

        if enabled || !has_only {
            progress_bar.update("initializing prisma");
            Docker::start_db("db", progress_bar)?;
            create_schema(progress_bar)?;
            copy_seed(progress_bar)?;
            migrate(progress_bar)?;
        }
        Ok(())
    }
}
