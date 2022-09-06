// #![allow(unused)]
extern crate log;
extern crate fs_extra;

mod cli_args;
mod commands;
mod logger;
mod lumberstack;
mod manifest;
mod sys_checks;
mod templates;
mod spinner;

use logger::Logger;
use lumberstack::Lumberstack;
use sys_checks::System;
use spinner::create_spinner;

fn main() {
    Logger::init();
    let spinner = create_spinner();
    System::check_prerequsites(&spinner);
    Lumberstack::run(&spinner);
}
