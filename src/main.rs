#![allow(unused)]
extern crate log;
extern crate colored;
mod cli;
mod configuration;
mod generators;
mod system;

use cli::{progress::*, logger::Logger};
use configuration::TemplateConfigFile;
use generators::{
    heroku::*, playwright::*, prisma::*, redwood::*, tailwind::Tailwind, templates::*,
};
use system::{error::AppError, utils::HandlebarBuilder, *};

fn main() -> Result<(), AppError> {
    Logger::init();
    let progress_bar = AppProgress::new();
    System::init(&progress_bar)?;
    Redwood::init(&progress_bar)?;
    Templates::init(&progress_bar)?;
    Tailwind::init(&progress_bar)?;
    Prisma::init(&progress_bar)?;
    Playwright::init(&progress_bar)?;
    Heroku::init(&progress_bar)?;
    Redwood::verify(&progress_bar)?;
    Redwood::cleanup(&progress_bar)?;
    progress_bar.finish("🚀 Setup finished");
    Ok(())
}
