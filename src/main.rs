// #![allow(unused)]
extern crate fs_extra;
extern crate log;

mod app_config;
mod cli_args;
mod commands;
mod framework;
mod lumberstack;
mod spinner;
mod system;
mod task_definitions;

use anyhow::Error;
use framework::redwood::{auth::RedwoodAuth, create::RedwoodApp, generate::RedwoodGenerate};
use lumberstack::Lumberstack;
use system::checks::System;
use task_definitions::templates::{copy::TemplateCopy, github::GithubTemplates, tags::TaskTag};

fn main() -> anyhow::Result<(), Error> {
    let app_config = System::init()?;
    let mut app = Lumberstack::new();

    let clone_task = GithubTemplates::create_runnable_task(TaskTag::Init, &app_config);
    let create_task = RedwoodApp::create_runnable_task(TaskTag::Create, &app_config);

    let generate_pages_task = RedwoodGenerate::create_runnable_task(TaskTag::Generate, &app_config);

    let auth_task = RedwoodAuth::create_runnable_task(TaskTag::Auth, &app_config);

    let template_copy_task = TemplateCopy::new(TaskTag::Templates, &app_config);

    app.queue(clone_task);
    app.queue(create_task);
    app.queue(generate_pages_task);
    app.queue(auth_task);
    app.queue(template_copy_task);

    app.process();

    Ok(())
}
