// #![allow(unused)]
extern crate fs_extra;
extern crate log;

mod cli_args;
mod commands;
mod default_config;
mod init;
mod logger;
mod lumberstack;
mod manifest;
mod spinner;
mod sys_checks;
mod templates;

use lumberstack::Lumberstack;
use manifest::Manifest;
use spinner::create_spinner;
use sys_checks::System;

pub static DEFAULT_TEMPLATE_DIR: &'static str = "templates";
pub static DEFAULT_APP_NAME: &'static str = "myapp";
pub static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.json";

fn main() {
    init::initialize();

    let spinner = create_spinner();
    System::check_prerequsites(&spinner);

    let manifest = Manifest::new();
    Lumberstack::run(&manifest, &spinner);

    spinner.set_prefix("✅");
    spinner.finish_with_message("Lumberstack Complete!");
}
