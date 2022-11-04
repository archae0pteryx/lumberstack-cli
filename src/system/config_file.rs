use std::collections::HashMap;

use crate::{
    app_config::DEFAULT_CONFIG_FILE,
    system::file_io::FileIO,
    task_definitions::templates::tags::{opt_tags_to_vec, TaskTag},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct ConfigFile {
    name: Option<String>,
    template_version: Option<String>,
    template_repo: Option<String>,
    vars: Option<HashMap<String, String>>,
    tags: Option<Vec<String>>,
    skip_tags: Option<Vec<String>>,
    log_file: Option<String>,
    #[serde(default = "bool::default")]
    skip_checks: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pages: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layouts: Option<Vec<String>>,
    #[serde(default = "bool::default")]
    clean: bool,
    #[serde(default = "bool::default")]
    save_playbook: bool,
    #[serde(default = "bool::default")]
    interactive: bool,
}

#[derive(Debug, Default)]
pub struct ParsedConfigFile {
    pub name: Option<String>,
    pub template_version: Option<String>,
    pub template_repo: Option<String>,
    pub vars: Option<HashMap<String, String>>,
    pub tags: Vec<TaskTag>,
    pub skip_tags: Vec<TaskTag>,
    pub log_file: Option<String>,
    pub skip_checks: bool,
    pub pages: Option<HashMap<String, String>>,
    pub layouts: Option<Vec<String>>,
    pub clean: bool,
    pub save_playbook: bool,
    pub interactive: bool,
}

impl ParsedConfigFile {}

pub fn load_config_from_file(config_path: Option<String>) -> Result<ParsedConfigFile> {
    let config_file = load_config_file(config_path)?;
    let parsed_config = parse_config_file(config_file)?;
    Ok(parsed_config)
}

fn parse_config_file(config_file: ConfigFile) -> Result<ParsedConfigFile> {
    let tags = opt_tags_to_vec(config_file.tags);
    let skip_tags = opt_tags_to_vec(config_file.skip_tags);
    Ok(ParsedConfigFile {
        name: config_file.name,
        template_version: config_file.template_version,
        template_repo: config_file.template_repo,
        vars: config_file.vars,
        tags,
        skip_tags,
        log_file: config_file.log_file,
        skip_checks: config_file.skip_checks,
        pages: config_file.pages,
        layouts: config_file.layouts,
        clean: config_file.clean,
        save_playbook: config_file.save_playbook,
        interactive: config_file.interactive,
    })
}

fn load_config_file(config: Option<String>) -> Result<ConfigFile> {
    let config_file_str =
        FileIO::read_or_skip(&config.unwrap_or_else(|| DEFAULT_CONFIG_FILE.to_string()));
    if let Some(c) = config_file_str {
        let config_file: ConfigFile = serde_yaml::from_str(&c)?;
        Ok(config_file)
    } else {
        Ok(ConfigFile::default())
    }
}
