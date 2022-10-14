// #![allow(unused)]
extern crate fs_extra;
extern crate log;

mod ansible;
mod cli_args;
mod commands;
mod logger;
mod lumberstack;
mod manifest;
mod redwood;
mod spinner;
mod system;
mod tags;
mod templates;
mod docker;

use anyhow::Error;
use log::error;
use logger::Logger;
use lumberstack::Lumberstack;
use manifest::Manifest;
use redwood::{create::RedwoodApp, auth::RedwoodAuth};
use std::process::{exit};
use system::{ init_system };
use tags::TaskTag;
use templates::{clone::TemplatesClone, parse::TemplateParser};

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.4";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
pub static DEFAULT_WORKDIR: &'static str = "tmp";
pub static DEFAULT_APP_NAME: &'static str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &'static str = "templates";
pub static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.yml";
pub static DEFAULT_LOG_FILE: &'static str = "lumberstack.out";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &'static str = "template_map.txt";
pub static DEFAULT_PLAYBOOK_FILE: &'static str = "playbook.yml";
pub static DEFAULT_ANSIBLE_TEMPLATE_REGEX: &'static str = r#"(\/\/|\/\/\*|#|\<!--) template!?.*"#;
// Rust regex specific
pub static TEMPLATE_TOKEN_REGEX: &'static str =
    r#"(//\*|//|#|<!--)\stemplate\[((?P<method>[^\]]+))\]"#;

fn main() -> anyhow::Result<(), Error> {
    Logger::init();

    let manifest = Manifest::load()?;



    match init_system(manifest.clone()) {
        Err(err) => {
            error!("{}", err.message);
            exit(exitcode::SOFTWARE);
        }
        _ => {}
    }

    let mut app = Lumberstack::new();

    let clone_task = TemplatesClone::new(TaskTag::Init, manifest.clone());
    let create_task = RedwoodApp::new(TaskTag::Create, manifest.clone());
    let auth_task = RedwoodAuth::new(TaskTag::Auth, manifest.clone());
    let parse_templates_task = TemplateParser::new(TaskTag::Parse, manifest.clone());

    app.queue(clone_task);
    app.queue(create_task);
    app.queue(auth_task);
    app.queue(parse_templates_task);
    app.process();
    Ok(())
}
