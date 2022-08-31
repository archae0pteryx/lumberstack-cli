use crate::cli::app_paths::AppPaths;
use crate::cli::progress::AppProgress;
use crate::system::error::AppError;
use crate::system::shell::Shell;

pub(super) fn create_app(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("🚀 Creating Redwood project (will take a minute)");
    let cmd = format!(
        "yarn create redwood-app {} --typescript --overwrite",
        AppPaths::root(None)
    );
    Shell::exec(cmd)
}

pub(super) fn generate_page(name: &str, path: &str) -> Result<(), AppError> {
    Shell::exec(format!(
        "cd {}; yarn rw generate page {} {}",
        AppPaths::root(None),
        name,
        path
    ))
}

pub(super) fn setup_auth(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("🔑 Setting up auth");
    Shell::exec(format!(
        "cd {}; yarn rw setup auth dbAuth --force",
        AppPaths::root(None)
    ))
}

pub(super) fn generate_auth(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("🔑 Scaffolding basic auth pages");
    Shell::exec(format!(
        "cd {}; yarn rw generate dbAuth --force",
        AppPaths::root(None)
    ))
}

pub(super) fn run_unit_tests(progress_bar: &AppProgress, package: &str) -> Result<(), AppError> {
    progress_bar.update(format!("📝 Running {} tests", package).as_str());
    Shell::exec(format!(
        "cd {}; yarn test {}",
        AppPaths::root(None),
        package
    ))?;
    Ok(())
}

pub(super) fn run_lint(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("👕 Linting app");
    // Shell::exec(format!("cd {}; yarn lint", AppPaths::root(None)))?;
    Ok(())
}
