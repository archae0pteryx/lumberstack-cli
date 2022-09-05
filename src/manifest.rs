use clap::Parser;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs};

use crate::cli_args::CliArgs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestJson {
    pub app_name: Option<String>,
    pub builder: Vec<BuildItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildItem {
    pub tag: String,
    pub feedback: String,
    pub context: Option<String>,
    pub commands: Option<Vec<CommandStep>>,
    pub templates: Option<Vec<TemplateItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateItem {
    pub feedback: Option<String>,
    pub source: String,
    pub dest: String,
    pub replace_map: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandStep {
    pub feedback: Option<String>,
    pub command: String,
    pub context: Option<String>,
}

pub fn load_manifest() -> ManifestJson {
    let args = CliArgs::parse();
    let config_to_load = args.config.unwrap_or("default_config.json".to_string());

    println!("loading config {}", &config_to_load);

    let rdr = fs::File::open(&config_to_load)
        .map_err(|_| error!("💣 Error opening config: {}", &config_to_load))
        .unwrap();

    let config_obj: ManifestJson = serde_json::from_reader(rdr)
        .map_err(|_| error!("💣 Error loading config as json: {}", &config_to_load))
        .unwrap();
    config_obj
}
