use crate::cli::app_paths::{AppPaths, TemplatePaths};
use crate::system::error::AppError;
use std::path::Path;
use std::path::PathBuf;

pub struct GithubActions;

impl GithubActions {
    pub fn copy_template() -> Result<(), AppError> {
        let source = TemplatePaths::root(Some(".github.template"));
        let dest = AppPaths::root(Some(".github"));
        let mut options = fs_extra::dir::CopyOptions::new();
        options.copy_inside = true;
        options.overwrite = true;
        fs_extra::dir::copy(&source, &dest, &options).map_err(|err| AppError {
            message: format!(
                "[github::copy_template] ❌ Error copying [{}] to [{}]. Error: {}",
                source, dest, err
            ),
        })?;
        Ok(())
    }
}
