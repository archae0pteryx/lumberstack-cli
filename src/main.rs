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

use anyhow::Error;
use lumberstack::Lumberstack;
use system::checks::System;
use task_definitions::{
    prisma::create::Prisma,
    redwood::{auth::RedwoodAuth, create::RedwoodApp, generate::RedwoodGenerate},
    templates::{copy::TemplateCopy, github::GithubTemplates, tags::TaskTag},
};

fn main() -> anyhow::Result<(), Error> {
    let app_config = System::init()?;
    let mut app = Lumberstack::new();

    let clone_task = GithubTemplates::clone_templates(TaskTag::Init, &app_config);
    let create_task = RedwoodApp::create_redwood_app(TaskTag::Create, &app_config);

    let generate_pages_task = RedwoodGenerate::generate_pages(TaskTag::Generate, &app_config);

    let auth_task = RedwoodAuth::generate_auth(TaskTag::Auth, &app_config);

    let template_copy_task = TemplateCopy::inject_templates(TaskTag::Templates, &app_config);

    let docker_task = Prisma::setup_prisma(TaskTag::Docker, &app_config);


    app.queue(clone_task);
    app.queue(create_task);
    app.queue(generate_pages_task);
    app.queue(auth_task);
    app.queue(template_copy_task);
    app.queue(docker_task);

    app.process();

    Ok(())
}
