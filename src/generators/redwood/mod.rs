use clap::Parser;

use crate::{
    cli::{
        arguments::{has_only_enabled, is_enabled, CliArgs, CliOptions},
        logger::Logger,
        progress::AppProgress,
    },
    system::error::AppError,
};

use self::{cmds::*, templates::*};

use super::{docker::Docker, playwright::Playwright};

mod cmds;
mod templates;
pub struct Redwood;

impl Redwood {
    pub fn init(progress_bar: &AppProgress) -> Result<(), AppError> {
        let args = CliArgs::parse();
        if args.not_redwood {
            Logger::loud_info(String::from("Skipping Redwood creation"));
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
        copy_components(progress_bar)?;
        copy_home_page()?;
        copy_layouts()?;
        copy_auth_unit_test()?;
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
            run_lint(progress_bar)?;
            Playwright::test(progress_bar)?;
        }
        Ok(())
    }

    pub fn cleanup(progress_bar: &AppProgress) -> Result<(), AppError> {
        progress_bar.update("🧹 Cleaning up lumberstack");
        Docker::cleanup(&progress_bar)?;
        Ok(())
    }
}
