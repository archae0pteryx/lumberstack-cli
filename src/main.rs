#![allow(unused)]
extern crate fs_extra;
extern crate log;
extern crate mockall;

// use clap::Parser;
use mockall::*;
use mockall::predicate::*;

mod cli_args;
mod commands;
mod logger;
mod lumberstack;
mod manifest;
mod spinner;
mod system;
mod task_definitions;
mod framework;
mod engine;
mod app_config;

use std::collections::HashMap;

use anyhow::Error;
use cli_args::{CliArgs, ParsedArgs};
use framework::redwood::{create::RedwoodApp, auth::RedwoodAuth};
use logger::Logger;
use lumberstack::Lumberstack;
use manifest::Manifest;
use serde_json::Value;
use system::System;
use task_definitions::templates::{clone::TemplatesClone, tags::TaskTag, copy::TemplateCopy};

fn main() -> anyhow::Result<(), Error> {
    // initialize system
    // Logger::init();

    // let manifest = Manifest::load()?;

    // System::init(manifest.clone());

    // let mut app = Lumberstack::new();

    // let clone_task = TemplatesClone::new(TaskTag::Init, manifest.clone());
    // let create_task = RedwoodApp::new(TaskTag::Create, manifest.clone());
    // let auth_task = RedwoodAuth::new(TaskTag::Auth, manifest.clone());
    // let template_copy_task = TemplateCopy::new(TaskTag::Templates, manifest.clone());

    // app.queue(clone_task);
    // app.queue(create_task);
    // app.queue(auth_task);
    // app.queue(template_copy_task);
    // app.process();

    Ok(())
}
