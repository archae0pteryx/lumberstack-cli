use std::collections::HashMap;

use crate::{
    system::{cli_args::ParsedArgs, file_io::FileIO},
    task_definitions::templates::tags::extract_all_tags,
};
use anyhow::Result;
use log::debug;
use phf::phf_map;
use serde::{Deserialize, Serialize};

pub static DEFAULT_TEMPLATE_VERSION: &str = "v0.0.4";
pub static DEFAULT_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
pub static DEFAULT_WORKDIR: &str = "tmp";
pub static DEFAULT_APP_NAME: &str = "myapp";
pub static DEFAULT_TEMPLATE_DIR: &str = "templates";
pub static DEFAULT_CONFIG_FILE: &str = "lumberstack.yml";
pub static DEFAULT_TEMPLATE_PATHS_FILE: &str = "template_map.txt";
pub static DEFAULT_PLAYBOOK_FILE: &str = "playbook.yml";
pub static DEFAULT_ANSIBLE_TEMPLATE_REGEX: &str = r#"(\/\/|\/\/\*|#|\<!--) template!?.*"#;
// Rust regex specific
pub static TEMPLATE_TOKEN_REGEX: &str = r#"(//\*|//|#|<!--)\stemplate\[((?P<method>[^\]]+))\]"#;

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
    pub generate_pages: HashMap<String, String>,
    pub generate_layouts: Vec<String>,
    pub clean: bool,
    pub save_playbook: bool,
    pub interactive: bool,
    pub avail_tags: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let workdir = DEFAULT_WORKDIR.to_string();
        let template_dir = format!("{}/{}", workdir, DEFAULT_TEMPLATE_DIR);
        let template_map = format!("{}/{}", workdir, DEFAULT_TEMPLATE_PATHS_FILE);
        AppConfig {
            app_name: DEFAULT_APP_NAME.to_string(),
            template_version: DEFAULT_TEMPLATE_VERSION.to_string(),
            tags: None,
            skip_tags: None,
            replace_vars: HashMap::new(),
            template_repo: DEFAULT_TEMPLATE_REPO.to_string(),
            template_dir,
            template_map,
            log_file: None,
            workdir,
            skip_checks: false,
            clean: false,
            save_playbook: false,
            interactive: true,
            avail_tags: Vec::new(),
            generate_pages: HashMap::new(),
            generate_layouts: Vec::new(),
        }
    }
}

pub fn load_app_config() -> Result<AppConfig> {
    let args = ParsedArgs::new();
    let config_file = load_config_file(args.config.clone())?;
    let processed_config = process_config(args, config_file).unwrap();
    debug!("AppConfig: {:#?}", processed_config);
    Ok(processed_config)
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

fn process_config(args: ParsedArgs, config_file: ConfigFile) -> Result<AppConfig> {
    let app_name = select_or_default_string(args.name, config_file.name, DEFAULT_APP_NAME);
    let template_version = select_or_default_string(
        args.template_version,
        config_file.template_version,
        DEFAULT_TEMPLATE_VERSION,
    );
    let template_repo = config_file
        .template_repo
        .unwrap_or_else(|| DEFAULT_TEMPLATE_REPO.to_string());
    let tags = args.tags.or(config_file.tags);
    let skip_tags = args.skip_tags.or(config_file.skip_tags);
    let replace_vars = combine_replace_vars(&app_name, config_file.vars);

    // let workdir = DEFAULT_WORKDIR.to_string();
    // let template_dir = format!("{}/{}", workdir, DEFAULT_TEMPLATE_DIR);
    // let template_map = format!("{}/{}", workdir, DEFAULT_TEMPLATE_PATHS_FILE);
    let log_file = select_or_none(args.log_file, config_file.log_file);
    let skip_checks = args.skip_checks || config_file.skip_checks;
    let pages = rw_pages_to_generate(config_file.pages);
    let layouts = rw_layouts_to_generate(config_file.layouts);
    let clean = config_file.clean || args.clean;
    let save_playbook = config_file.save_playbook;
    // let interactive = args.interactive || config_file.interactive;
    // let interactive = true;
    let template_map = AppConfig::default().template_map;
    let avail_tags = extract_all_tags(&template_map)?;

    Ok(AppConfig {
        app_name,
        template_version,
        template_repo,
        tags,
        skip_tags,
        replace_vars,
        template_map,
        log_file,
        skip_checks,
        generate_pages: pages,
        generate_layouts: layouts,
        clean,
        save_playbook,
        avail_tags,
        ..AppConfig::default()
    })
}

fn rw_pages_to_generate(config_pages: Option<HashMap<String, String>>) -> HashMap<String, String> {
    let mut pages = HashMap::new();
    for (name, path) in DEFAULT_PAGES.into_iter() {
        pages.insert(name.to_string(), path.to_string());
    }

    if let Some(config_pages) = config_pages {
        for (name, path) in config_pages.into_iter() {
            pages.insert(name, path);
        }
    }

    pages
}

fn rw_layouts_to_generate(config_layouts: Option<Vec<String>>) -> Vec<String> {
    let mut layouts = Vec::new();
    for default_layout in DEFAULT_LAYOUTS {
        layouts.push(default_layout.to_string());
    }

    if let Some(config_layouts) = config_layouts {
        for layout in config_layouts {
            layouts.push(layout);
        }
    }
    layouts
}

fn combine_replace_vars(
    app_name: &str,
    config_template_vars: Option<HashMap<String, String>>,
) -> HashMap<String, String> {
    let mut template_vars = HashMap::new();
    template_vars.insert("app_name".to_string(), app_name.to_owned());

    if let Some(vars) = config_template_vars {
        for (key, value) in vars {
            template_vars.insert(key, value);
        }
    }
    template_vars
}

#[derive(Serialize, Deserialize, Clone, Default)]
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

fn select_or_default_string(s1: Option<String>, s2: Option<String>, default: &str) -> String {
    s1.unwrap_or_else(|| s2.unwrap_or_else(|| default.to_string()))
}

fn select_or_none(opt_a: Option<String>, opt_b: Option<String>) -> Option<String> {
    opt_a.or(opt_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_app_config() {
        let args = ParsedArgs::default();

        let config_file = ConfigFile::default();

        let actual = process_config(args, config_file).unwrap();

        assert_eq!(actual.app_name, DEFAULT_APP_NAME.to_string());
        assert_eq!(
            actual.template_version,
            DEFAULT_TEMPLATE_VERSION.to_string()
        );
        assert_eq!(actual.template_repo, DEFAULT_TEMPLATE_REPO.to_string());
        assert_eq!(actual.tags, None);
        assert_eq!(actual.skip_tags, None);
        assert_eq!(
            actual.replace_vars.get("app_name").unwrap(),
            &DEFAULT_APP_NAME.to_string()
        );

        dbg!(&actual);
    }
}
