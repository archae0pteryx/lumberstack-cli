use self::{cmds::*, templates::*};
use super::{docker::Docker, playwright::Playwright};
use crate::{
    cli::{
        arguments::{has_only_enabled, is_enabled, CliArgs, CliOptions},
        progress::AppProgress,
    },
    system::error::AppError,
};
use clap::Parser;
use log::info;

mod cmds;
mod templates;
pub struct Redwood;

impl Redwood {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        let args = CliArgs::parse();
        if args.not_redwood {
            info!("Skipping Redwood creation");
            return Ok(());
        }

        if is_enabled(&CliOptions::Redwood) || !has_only_enabled() {
            create_app(progress_bar)?;
            Self::run_generators(progress_bar)?;
            Self::copy_templates(progress_bar)?;
        }
        Ok(())
    }

    fn copy_templates(progress_bar: &AppProgress) -> Result<(), AppError> {
        if is_enabled(&CliOptions::Redwood)
            || is_enabled(&CliOptions::Templates)
            || !has_only_enabled()
        {
            copy_components(progress_bar)?;
            copy_home_page()?;
            copy_layouts()?;
            copy_auth_unit_test()?;
        }
        Ok(())
    }

    fn run_generators(progress_bar: &AppProgress) -> Result<(), AppError> {
        generate_page("home", "/")?;
        setup_auth(progress_bar)?;
        generate_auth(progress_bar)?;
        Ok(())
    }

    pub fn verify(progress_bar: &AppProgress) -> Result<(), AppError> {
        if !has_only_enabled() {
            run_unit_tests(progress_bar, "web")?;
            run_unit_tests(progress_bar, "api")?;
            Playwright::test(progress_bar)?;
        }
        Ok(())
    }

    pub fn cleanup(progress_bar: &AppProgress) -> Result<(), AppError> {
        progress_bar.update("🧹 Cleaning up installation");
        Docker::cleanup(&progress_bar)?;
        Ok(())
    }
}
