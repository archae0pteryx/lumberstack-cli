// #![allow(unused)]
extern crate fs_extra;
extern crate log;

mod ansible;
mod cli_args;
mod commands;
mod logger;
mod lumberstack;
mod manifest;
mod spinner;
mod system;

use logger::Logger;
use lumberstack::Lumberstack;
use manifest::Manifest;
use spinner::create_spinner;
use system::System;

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.3";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";

pub static DEFAULT_WORKDIR: &'static str = "/tmp/lumberstack";
pub static DEFAULT_APP_NAME: &'static str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &'static str = "redwood-template-app";
pub static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.json";
pub static DEFAULT_LOG_FILE: &'static str = "lumberstack.out";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &'static str = "template_map.txt";

fn main() -> anyhow::Result<()> {
    Logger::init();
    let spinner = create_spinner();
    let manifest = Manifest::load()?;

    System::init(&manifest, &spinner);

    Lumberstack::start(&manifest,  &spinner);

    spinner.set_prefix("✅");
    spinner.finish_with_message("Lumberstack Complete!");
    Ok(())
}
