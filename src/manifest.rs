use clap::Parser;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs};

use crate::cli_args::CliArgs;

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
        let manifest_str = Self::read_manifest();
        let handlebars = Handlebars::new();
        let manifest: ManifestJson =
            serde_json::from_str(&manifest_str).expect("Error reading json");
        let out = handlebars
            .render_template(&manifest_str, &manifest)
            .expect("Error rendering template");

        let processesed_manifest: ManifestJson =
            serde_json::from_str(&out).expect("Error loading JSON");

        Manifest {
            app_name: Self::app_name(&manifest),
            json: processesed_manifest,
        }
    }

    fn read_manifest() -> String {
        let args = CliArgs::parse();
        let config = args.config.unwrap_or(DEFAULT_MANIFEST_FILE.to_string());
        fs::read_to_string(config).expect("Cant load manifest config")
    }

    // Prefer name from args
    pub fn app_name(config: &ManifestJson) -> String {
        let args = CliArgs::parse();
        match &args.name {
            Some(name) => {
                return name.to_owned();
            }
            None => match &config.app_name {
                Some(name) => {
                    return String::from(name);
                }
                None => return String::from(DEFAULT_APP_NAME),
            },
        }
    }
}
