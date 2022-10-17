// #![allow(unused)]
extern crate fs_extra;
extern crate log;
extern crate mockall;

use app_config::{load_app_config};

mod app_config;
mod cli_args;
mod commands;
mod engine;
mod framework;
mod logger;
mod lumberstack;
mod manifest;
mod spinner;
mod system;
mod task_definitions;

use anyhow::Error;
use logger::Logger;
use lumberstack::Lumberstack;
use system::System;
use task_definitions::templates::{clone::TemplatesClone, tags::TaskTag};

fn main() -> anyhow::Result<(), Error> {
    // initialize system
    Logger::init();
    System::init();

    let app_config = load_app_config()?;

    let mut app = Lumberstack::new();

    let clone_task = TemplatesClone::new(TaskTag::Init, &app_config);
    // let create_task = RedwoodApp::new(TaskTag::Create, manifest.clone());
    // let auth_task = RedwoodAuth::new(TaskTag::Auth, manifest.clone());
    // let template_copy_task = TemplateCopy::new(TaskTag::Templates, manifest.clone());

    app.queue(clone_task);
    // app.queue(create_task);
    // app.queue(auth_task);
    // app.queue(template_copy_task);
    app.process();

    Ok(())
}
