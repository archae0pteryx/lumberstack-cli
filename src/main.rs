#![allow(unused)]
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
mod templates;

use logger::Logger;
use lumberstack::Lumberstack;
use manifest::Manifest;
use spinner::create_spinner;
use system::System;

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.3";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";

pub static DEFAULT_WORKDIR: &'static str = "tmp";
pub static DEFAULT_APP_NAME: &'static str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &'static str = "templates";
pub static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.yml";
pub static DEFAULT_LOG_FILE: &'static str = "lumberstack.out";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &'static str = "template_map.txt";
pub static DEFAULT_PLAYBOOK_FILE: &'static str = "playbook.yml";

fn main() -> anyhow::Result<()> {
    Logger::init();
    let spinner = create_spinner();
    let manifest = Manifest::load()?;

    // fs_extra::dir::remove(DEFAULT_WORKDIR.to_string()).expect("error removing");

    System::init(manifest.clone(), &spinner);

    Lumberstack::start(manifest,  &spinner);

    spinner.set_prefix("✅");
    spinner.finish_with_message("Lumberstack Complete!");
    Ok(())
}
