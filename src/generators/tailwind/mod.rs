use crate::{
    cli::{app_paths::AppPaths, progress::AppProgress, arguments::has_only_enabled},
    system::{error::AppError, shell::Shell},
};

pub struct Tailwind;

impl Tailwind {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        let has_only = has_only_enabled();

        if !has_only {
            progress_bar.update("💨 Installing tailwind");
            Shell::exec(format!(
                "cd {}; yarn rw setup ui tailwind",
                AppPaths::root(None)
            ))?;
        }
        Ok(())
    }
}
