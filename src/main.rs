// #![allow(unused)]
extern crate fs_extra;
extern crate log;

mod app_config;
mod cli_args;
mod commands;
mod lumberstack;
mod spinner;
mod system;
mod task_definitions;
mod tasks;
mod ui;

use anyhow::{Error, Result};
use system::checks::init_system;

use tasks::execute_tasks;
use ui::start_ui::start_ui;

fn main() -> Result<(), Error> {
    let app_config = init_system()?;

    if app_config.interactive {
        start_ui(app_config)?;
    } else {
        execute_tasks(&app_config)?;
    }

    Ok(())
}
