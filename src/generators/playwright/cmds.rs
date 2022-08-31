use crate::{
    cli::{app_paths::AppPaths, progress::AppProgress},
    system::{error::AppError, shell::Shell}, generators::docker::Docker,
};

pub(super) fn install_playwright(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("🏠 Initializing Playwright");
    Shell::exec(format!(
        "cd {}; yarn create playwright --quiet --lang=ts",
        AppPaths::web(None)
    ))
}

pub(super) fn run_playwright_tests(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("🏠 Running Playwright E2E");
    Docker::start_db("testdb", progress_bar)?;
    Shell::exec(format!("cd {}; yarn test:e2e", AppPaths::root(None)))?;
    Ok(())
}
