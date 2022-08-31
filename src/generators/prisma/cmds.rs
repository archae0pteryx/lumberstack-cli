use crate::{
    cli::{app_paths::AppPaths, progress::AppProgress},
    system::{error::AppError, shell::Shell},
};

pub(super) fn migrate(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("💎 Running migrations");
    let command = format!(
        "cd {}; npx prisma migrate dev --name init",
        AppPaths::root(None)
    );
    Shell::exec(command)?;
    Ok(())
}
