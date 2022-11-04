// #![allow(unused)]
extern crate fs_extra;
extern crate log;

mod app_config;
mod lumberstack;
mod system;
mod task_definitions;
mod tasks;
mod ui;

use anyhow::{Error, Result};
use system::System;
use tasks::TaskEngine;
use ui::start_ui::start_ui;

fn main() -> Result<(), Error> {
    let app_config= System::init()?;

    if app_config.interactive {
        start_ui(app_config)?;
    } else {
        TaskEngine::execute(&app_config)?;
    }
    Ok(())
}
