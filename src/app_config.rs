use std::{collections::HashMap};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{cli_args::ParsedArgs, system::file_io::FileIO};

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.4";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
pub static DEFAULT_WORKDIR: &'static str = "tmp";
pub static DEFAULT_APP_NAME: &'static str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &'static str = "templates";
pub static DEFAULT_MANIFEST_FILE: &'static str = "lumberstack.yml";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &'static str = "template_map.txt";
pub static DEFAULT_PLAYBOOK_FILE: &'static str = "playbook.yml";
pub static DEFAULT_ANSIBLE_TEMPLATE_REGEX: &'static str = r#"(\/\/|\/\/\*|#|\<!--) template!?.*"#;
// Rust regex specific
pub static TEMPLATE_TOKEN_REGEX: &'static str =
    r#"(//\*|//|#|<!--)\stemplate\[((?P<method>[^\]]+))\]"#;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub template_version: String,
    pub tags: Option<Vec<String>>,
    pub skip_tags: Option<Vec<String>>,
    pub template_vars: HashMap<String, String>,
    pub template_repo: String,
    pub template_dir: String,
    pub template_map: String,
    pub log_file: Option<String>,
    pub workdir: String,
    pub skip_checks: bool
}

pub fn load_app_config() -> Result<AppConfig> {
    let args = ParsedArgs::new();
    let config_file = load_config_file(args.config.clone())?;
    let processed_config = process_config(args.clone(), config_file);
    debug!("AppConfig: {:#?}", processed_config);
    Ok(processed_config)
}

fn load_config_file(config: Option<String>) -> Result<ConfigFile> {
    let config_file_str =
        FileIO::read(&config.unwrap_or(DEFAULT_MANIFEST_FILE.to_string())).unwrap();
    let config: ConfigFile = serde_yaml::from_str(config_file_str.as_str())?;
    return Ok(config);
}

fn process_config(args: ParsedArgs, config_file: ConfigFile) -> AppConfig {
    let app_name = select_or_default_string(args.name, config_file.name, DEFAULT_APP_NAME);
    let template_version = select_or_default_string(
        args.template_version,
        config_file.template_version,
        DEFAULT_TEMPLATE_VERSION,
    );
    let template_repo = config_file
        .template_repo
        .unwrap_or(DEFAULT_TEMPLATE_REPO.to_string());
    let tags = args.tags.or(config_file.tags);
    let skip_tags = args.skip_tags.or(config_file.skip_tags);
    let template_vars = process_template_vars(&app_name, config_file.template_vars);

    let workdir = DEFAULT_WORKDIR.to_string();
    let template_dir = format!("{}/{}", workdir, DEFAULT_TEMPLATE_DIR);
    let template_map = format!("{}/{}", workdir, DEFAULT_TEMPLATE_PATHS_FILE);
    let log_file = select_or_none(args.log_file, config_file.log_file);
    let skip_checks = args.skip_checks || config_file.skip_checks.unwrap_or(false);
    AppConfig {
        app_name,
        template_version,
        template_repo,
        tags,
        skip_tags,
        template_vars,
        template_dir,
        template_map,
        log_file,
        workdir,
        skip_checks
    }
}

fn process_template_vars(
    app_name: &String,
    config_template_vars: Option<HashMap<String, String>>,
) -> HashMap<String, String> {
    let mut template_vars = HashMap::new();
    template_vars.insert("$app_name".to_string(), app_name.clone());

    if let Some(vars) = config_template_vars {
        for (key, value) in vars {
            template_vars.insert(key, value);
        }
    }
    template_vars
}

fn select_or_none(opt_a: Option<String>, opt_b: Option<String>) -> Option<String> {
    opt_a.or(opt_b)
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    name: Option<String>,
    template_version: Option<String>,
    template_repo: Option<String>,
    template_vars: Option<HashMap<String, String>>,
    tags: Option<Vec<String>>,
    skip_tags: Option<Vec<String>>,
    log_file: Option<String>,
    skip_checks: Option<bool>
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            name: None,
            template_version: None,
            template_repo: None,
            skip_tags: None,
            tags: None,
            template_vars: None,
            log_file: None,
            skip_checks: None
        }
    }
}

fn select_or_default_string(s1: Option<String>, s2: Option<String>, default: &str) -> String {
    s1.unwrap_or(s2.unwrap_or(default.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_app_config() {
        let args = ParsedArgs::default();

        let config_file = ConfigFile::default();

        let actual = process_config(args, config_file);

        assert_eq!(actual.app_name, DEFAULT_APP_NAME.to_string());
        assert_eq!(
            actual.template_version,
            DEFAULT_TEMPLATE_VERSION.to_string()
        );
        assert_eq!(actual.template_repo, DEFAULT_TEMPLATE_REPO.to_string());
        assert_eq!(actual.tags, None);
        assert_eq!(actual.skip_tags, None);
        assert_eq!(
            actual.template_vars.get("$app_name").unwrap(),
            &DEFAULT_APP_NAME.to_string()
        );

        dbg!(&actual);
    }
}
