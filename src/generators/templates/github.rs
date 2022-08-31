use crate::cli::app_paths::{AppPaths, TemplatePaths};
use crate::system::error::AppError;
use crate::system::utils::copy_directory;
use std::path::PathBuf;

pub struct GithubActions;

impl GithubActions {
    pub fn copy_template() -> Result<(), AppError> {
        let source = PathBuf::from(TemplatePaths::root(Some(".github.template")));
        let destination = PathBuf::from(AppPaths::root(Some(".github")));
        copy_directory(source, destination)
    }
}
