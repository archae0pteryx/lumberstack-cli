use self::cmds::{install_playwright, run_playwright_tests};
use crate::{
    cli::{arguments::{is_enabled, CliOptions, has_only_enabled}, progress::AppProgress},
    system::error::AppError,
};

mod cmds;
pub struct Playwright;

impl Playwright {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        if is_enabled(&CliOptions::Playwright) || !has_only_enabled() {
            install_playwright(progress_bar)?;
        }
        Ok(())
    }

    pub fn test(progress_bar: &AppProgress) -> Result<(), AppError> {
        // start redwood
        run_playwright_tests(progress_bar)?;
        Ok(())
    }
}
