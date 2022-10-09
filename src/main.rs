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

use logger::Logger;
use lumberstack::Lumberstack;
use manifest::Manifest;
use system::System;
use tags::TaskTag;
use templates::init::TemplatesInit;

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.4-1";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
pub static DEFAULT_WORKDIR: &'static str = "tmp";
pub static DEFAULT_APP_NAME: &'static str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &'static str = "templates";
pub static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.yml";
pub static DEFAULT_LOG_FILE: &'static str = "lumberstack.out";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &'static str = "template_map.txt";
pub static DEFAULT_PLAYBOOK_FILE: &'static str = "playbook.yml";
pub static TEMPLATE_TOKEN_REGEX: &'static str =
    r#"(//\*|//|#|<!--)\stemplate\[((?P<method>[^\]]+))\]"#;

fn main() -> anyhow::Result<()> {
    Logger::init();
    let manifest = Manifest::load()?;

    System::init(manifest.clone());

    let mut app = Lumberstack::new();

    let init_task = TemplatesInit::new(TaskTag::Init, manifest.clone());

    app.queue(init_task);
    app.process();

    Ok(())
}
