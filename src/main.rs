#![allow(unused)]
extern crate fs_extra;
extern crate log;

mod app_config;
mod cli_args;
mod commands;
mod lumberstack;
mod spinner;
mod system;
mod task_definitions;
mod ui;

use anyhow::Error;
use app_config::AppConfig;
use lumberstack::Lumberstack;
use std::sync::{Arc, Mutex};
use system::checks::init_system;
use task_definitions::{
    heroku::create::Heroku,
    prisma::create::Prisma,
    redwood::{
        auth::RedwoodAuth, create::RedwoodApp, generate::RedwoodGenerate, playwright::Playwright,
        tailwind::Tailwind,
    },
    templates::{copy::TemplateCopy, github::GithubTemplates, tags::TaskTag},
};
use ui::start_ui::start_ui;

fn main() -> anyhow::Result<(), Error> {
    let app_config = init_system()?;
    let app_config = Box::new(app_config);
    if app_config.interactive {
        start_ui(app_config);
    } else {
        // execute_tasks(&app_config)?;
    }

    Ok(())
}

fn execute_tasks(app_config: &AppConfig) -> anyhow::Result<()> {
    let mut app = Lumberstack::new();

    let clone_task = GithubTemplates::clone_templates(TaskTag::Clone, &app_config);

    let create_task = RedwoodApp::create_redwood_app(TaskTag::Create, &app_config);

    let playwright_task = Playwright::create_playwright(TaskTag::Playwright, &app_config);

    let generate_pages_task = RedwoodGenerate::generate_pages(TaskTag::Generate, &app_config);

    let auth_task = RedwoodAuth::generate_auth(TaskTag::Auth, &app_config);

    let tailwind_task = Tailwind::create_tailwind(TaskTag::Tailwind, &app_config);

    let template_copy_task = TemplateCopy::inject_templates(TaskTag::Templates, &app_config);

    let prisma_task = Prisma::setup_prisma(TaskTag::Prisma, &app_config);

    let heroku_task = Heroku::create_heroku(TaskTag::Heroku, &app_config);

    app.queue(clone_task);
    app.queue(create_task);
    app.queue(tailwind_task);
    app.queue(playwright_task);
    app.queue(generate_pages_task);
    app.queue(auth_task);
    app.queue(template_copy_task);
    app.queue(prisma_task);
    app.queue(heroku_task);

    app.process();
    Ok(())
}
