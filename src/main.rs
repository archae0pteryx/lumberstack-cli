// #![allow(unused)]
extern crate colored;
extern crate log;

mod logger;
mod cli_args;
mod lumberstack;
mod sys_checks;
mod progress;
mod exec_command;
mod manifest;

use logger::Logger;
use lumberstack::Lumberstack;
use sys_checks::System;

fn main() {
    Logger::init();
    System::check_prerequsites();
    Lumberstack::run();
}
