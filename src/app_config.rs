use std::collections::HashMap;

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{cli_args::ParsedArgs, system::file_io::FileIO};
use phf::phf_map;

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.4";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
pub static DEFAULT_WORKDIR: &str = "tmp";
pub static DEFAULT_APP_NAME: &str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &str = "templates";
pub static DEFAULT_MANIFEST_FILE: &str = "lumberstack.yml";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &str = "template_map.txt";
pub static DEFAULT_PLAYBOOK_FILE: &str = "playbook.yml";
pub static DEFAULT_ANSIBLE_TEMPLATE_REGEX: &str = r#"(\/\/|\/\/\*|#|\<!--) template!?.*"#;
// Rust regex specific
pub static TEMPLATE_TOKEN_REGEX: &str =
    r#"(//\*|//|#|<!--)\stemplate\[((?P<method>[^\]]+))\]"#;

pub static DEFAULT_PAGES: phf::Map<&str, &str> = phf_map! {
    "home" => "/",
    "about" => "/about",
};

pub static DEFAULT_LAYOUTS: &[&str] = &["admin"];

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub template_version: String,
    pub tags: Option<Vec<String>>,
    pub skip_tags: Option<Vec<String>>,
    pub replace_vars: HashMap<String, String>,
    pub template_repo: String,
    pub template_dir: String,
    pub template_map: String,
    pub log_file: Option<String>,
    pub workdir: String,
    pub skip_checks: bool,
    pub pages: HashMap<String, String>,
    pub layouts: Vec<String>,
    pub clean: bool,
    pub save_playbook: bool,
}

pub fn load_app_config() -> Result<AppConfig> {
    let args = ParsedArgs::new();
    let config_file = load_config_file(args.config.clone())?;
    let processed_config = process_config(args, config_file);
    debug!("AppConfig: {:#?}", processed_config);
    Ok(processed_config)
}

fn load_config_file(config: Option<String>) -> Result<ConfigFile> {
    let config_file_str =
        FileIO::read(&config.unwrap_or(DEFAULT_MANIFEST_FILE.to_string())).unwrap();
    let config: ConfigFile = serde_yaml::from_str(config_file_str.as_str())?;
    Ok(config)
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
    let replace_vars = combine_replace_vars(&app_name, config_file.vars);

    let workdir = DEFAULT_WORKDIR.to_string();
    let template_dir = format!("{}/{}", workdir, DEFAULT_TEMPLATE_DIR);
    let template_map = format!("{}/{}", workdir, DEFAULT_TEMPLATE_PATHS_FILE);
    let log_file = select_or_none(args.log_file, config_file.log_file);
    let skip_checks = args.skip_checks || config_file.skip_checks.unwrap_or(false).to_owned();
    let pages = pages_to_generate(config_file.pages);
    let layouts = layouts_to_generate(config_file.layouts);
    let clean = config_file.clean || args.clean;
    let save_playbook = config_file.save_playbook;
    AppConfig {
        app_name,
        template_version,
        template_repo,
        tags,
        skip_tags,
        replace_vars,
        template_dir,
        template_map,
        log_file,
        workdir,
        skip_checks,
        pages,
        layouts,
        clean,
        save_playbook
    }
}

fn pages_to_generate(config_pages: Option<HashMap<String, String>>) -> HashMap<String, String> {
    let mut pages = HashMap::new();
    for (name, path) in DEFAULT_PAGES.into_iter() {
        pages.insert(name.to_string(), path.to_string());
    }

    if config_pages.is_some() {
        for (key, value) in config_pages.unwrap().iter() {
            pages.insert(key.to_string(), value.to_string());
        }
    }

    if pages.is_empty() {
        return HashMap::new();
    }
    pages
}

fn layouts_to_generate(config_layouts: Option<Vec<String>>) -> Vec<String> {
    let mut layouts = Vec::new();
    for default_layout in DEFAULT_LAYOUTS {
        layouts.push(default_layout.to_string());
    }
    if config_layouts.is_some() {
        for layout in config_layouts.unwrap().iter() {
            layouts.push(layout.to_string());
        }
    }
    layouts
}

fn combine_replace_vars(
    app_name: &String,
    config_template_vars: Option<HashMap<String, String>>,
) -> HashMap<String, String> {
    let mut template_vars = HashMap::new();
    template_vars.insert("app_name".to_string(), app_name.clone());

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

#[derive(Serialize, Deserialize, Clone)]
#[derive(Default)]
struct ConfigFile {
    name: Option<String>,
    template_version: Option<String>,
    template_repo: Option<String>,
    vars: Option<HashMap<String, String>>,
    tags: Option<Vec<String>>,
    skip_tags: Option<Vec<String>>,
    log_file: Option<String>,
    skip_checks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pages: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layouts: Option<Vec<String>>,
    clean: bool,
    save_playbook: bool,
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
            actual.replace_vars.get("$app_name").unwrap(),
            &DEFAULT_APP_NAME.to_string()
        );

        dbg!(&actual);
    }
}
