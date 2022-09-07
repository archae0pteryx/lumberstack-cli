use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs};

use crate::{cli_args::CliArgs, default_config::generate_default_config};

static DEFAULT_APP_NAME: &'static str = "myapp";
static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManifestJson {
    pub app_name: Option<String>,
    pub builder: Vec<BuildItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Debug, Clone)]

pub struct Manifest {
    pub app_name: String,
    pub json: ManifestJson,
}

impl Manifest {
    pub fn new() -> Manifest {
        let tmp_app_name = Self::tmp_app_name();
        let manifest_str = Self::read_manifest();

        let processed_manifest = manifest_str.replace("{{app_name}}", &tmp_app_name);

        let manifest: ManifestJson =
        serde_json::from_str(&processed_manifest).expect("Error reading json");

        let app_name = manifest.app_name.clone().unwrap_or(DEFAULT_APP_NAME.to_string());

        Manifest {
            app_name,
            json: manifest,
        }
    }

    fn read_manifest() -> String {
        let args = CliArgs::parse();
        match &args.config {
            Some(conf) => {
                let config = fs::read_to_string(conf).expect("Error reading users manifest");

                return config;
            }
            None => {
                if fs::metadata(DEFAULT_MANIFEST_FILE).is_ok() {
                    return fs::read_to_string(DEFAULT_MANIFEST_FILE)
                        .expect("Error reading default manifest");
                }
                return generate_default_config().to_string();
            }
        }
    }

    pub fn tmp_app_name() -> String {
        let args = CliArgs::parse();
        match &args.name {
            Some(name) => {
                return name.to_owned();
            }
            None => String::from(DEFAULT_APP_NAME),
        }
    }
}
