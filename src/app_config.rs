use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::cli_args::ParsedArgs;

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


#[derive(Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub template_version: String,
    pub tags: Option<Vec<String>>,
    pub skip_tags: Option<Vec<String>>,
    pub template_vars: HashMap<String, String>,
    pub template_repo: String,
}

pub fn load_app_config() -> Result<AppConfig> {
    let args = ParsedArgs::new();
    let config_file = load_config_file(args.config.clone())?;
    let generated_config = generate_app_config(args.clone(), config_file);
    Ok(generated_config)
}

fn load_config_file(config: Option<String>) -> Result<ConfigFile> {
    let config_file_str =
        fs_extra::file::read_to_string(config.unwrap_or(DEFAULT_MANIFEST_FILE.to_string()))?;
    let config: ConfigFile = serde_yaml::from_str(config_file_str.as_str())?;
    return Ok(config);
}

fn generate_app_config(args: ParsedArgs, config_file: ConfigFile) -> AppConfig {
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
    let mut default_template_vars = HashMap::new();
    default_template_vars.insert("$app_name".to_string(), app_name.clone());
    let template_vars = config_file.template_vars.unwrap_or(default_template_vars);
    AppConfig {
        app_name,
        template_version,
        template_repo,
        tags,
        skip_tags,
        template_vars,
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    name: Option<String>,
    template_version: Option<String>,
    template_repo: Option<String>,
    template_vars: Option<HashMap<String, String>>,
    tags: Option<Vec<String>>,
    skip_tags: Option<Vec<String>>,
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

        let actual = generate_app_config(args, config_file);

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
