use clap::Parser;
use serde_json::Value;
use std::{collections::HashMap, fs};

use crate::{
    cli_args::CliArgs, DEFAULT_APP_NAME, DEFAULT_LOG_FILE, DEFAULT_TEMPLATE_DIR,
    DEFAULT_TEMPLATE_MAP, DEFAULT_TEMPLATE_REPO, DEFAULT_TEMPLATE_VERSION, DEFAULT_WORKDIR
};

pub struct Manifest {
    pub app_name: String,
    pub template_version: String,
    pub config_file: HashMap<String, Value>,
    pub workdir: String,
    pub clean: bool,
    pub template_repo: String,
    pub template_dir: String,
    pub template_map: String,
    pub log_file: String,
}

impl Manifest {
    pub fn load() -> Manifest {
        let args = CliArgs::parse();
        let template_version = args
            .clone()
            .template_version
            .unwrap_or(DEFAULT_TEMPLATE_VERSION.to_string());
        let app_name = args.clone().name.unwrap_or(DEFAULT_APP_NAME.to_string());
        Manifest {
            app_name,
            template_version,
            config_file: Self::config_file(&args),
            template_repo: DEFAULT_TEMPLATE_REPO.to_string(),
            workdir: DEFAULT_WORKDIR.to_string(),
            template_dir: format!("{}/{}", DEFAULT_WORKDIR, DEFAULT_TEMPLATE_DIR),
            template_map: format!("{}/{}", DEFAULT_WORKDIR, DEFAULT_TEMPLATE_MAP),
            log_file: format!("{}/{}", DEFAULT_WORKDIR, DEFAULT_LOG_FILE),
            clean: args.clean
        }
    }

    fn config_file(args: &CliArgs) -> HashMap<String, Value> {
        if let Some(config_path) = &args.config {
            return Self::load_config_file(config_path);
        }
        HashMap::new()
    }

    fn load_config_file(config_path: &String) -> HashMap<String, Value> {
        let err_msg = format!("Couldnt load config from {}", config_path);
        let loaded = fs::read_to_string(config_path).expect(&err_msg);
        let raw_config_json: HashMap<String, Value> =
            serde_json::from_str(&loaded).expect(&err_msg);
        return raw_config_json;
    }
}
