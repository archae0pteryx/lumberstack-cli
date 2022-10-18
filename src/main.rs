// #![allow(unused)]
extern crate fs_extra;
extern crate log;

use app_config::{load_app_config};

mod app_config;
mod cli_args;
mod commands;
mod file_io;
mod framework;
mod logger;
mod lumberstack;
mod manifest;
mod spinner;
mod system;
mod task_definitions;

use anyhow::Error;
use framework::redwood::{create::RedwoodApp, auth::RedwoodAuth};
use logger::Logger;
use lumberstack::Lumberstack;
use system::System;
use task_definitions::templates::{clone::TemplatesClone, tags::TaskTag, copy::TemplateCopy};

fn main() -> anyhow::Result<(), Error> {
    Logger::init();

    let app_config = load_app_config()?;

    System::init(&app_config)?;


    let mut app = Lumberstack::new();

    let clone_task = TemplatesClone::new(TaskTag::Init, &app_config);
    let create_task = RedwoodApp::new(TaskTag::Create, &app_config);
    let auth_task = RedwoodAuth::new(TaskTag::Auth, &app_config);
    let template_copy_task = TemplateCopy::new(TaskTag::Templates, &app_config);

    app.queue(clone_task);
    app.queue(create_task);
    app.queue(auth_task);
    app.queue(template_copy_task);

    app.process();

    Ok(())
}
