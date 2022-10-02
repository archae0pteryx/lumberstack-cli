use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    cli_args::CliArgs, DEFAULT_APP_NAME, DEFAULT_LOG_FILE, DEFAULT_TEMPLATE_DIR,
    DEFAULT_TEMPLATE_PATHS_FILE, DEFAULT_TEMPLATE_REPO, DEFAULT_TEMPLATE_VERSION, DEFAULT_WORKDIR,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub app_name: Option<String>,
    pub template_version: Option<String>,
    pub workdir: Option<String>,
    pub clean: Option<bool>,
    pub template_repo: Option<String>,
    pub template_dir: Option<String>,
    pub template_paths_file: Option<String>,
    pub log_file: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest {
            app_name: Some(DEFAULT_APP_NAME.to_string()),
            template_version: Some(DEFAULT_TEMPLATE_VERSION.to_string()),
            workdir: Some(DEFAULT_WORKDIR.to_string()),
            clean: Some(true),
            template_repo: Some(DEFAULT_TEMPLATE_REPO.to_string()),
            template_dir: Some(DEFAULT_TEMPLATE_DIR.to_string()),
            template_paths_file: Some(DEFAULT_TEMPLATE_PATHS_FILE.to_string()),
            log_file: Some(DEFAULT_LOG_FILE.to_string()),
            tags: Some(Vec::new()),
        }
    }
}

impl Manifest {
    pub fn load() -> Result<Manifest> {
        let args = CliArgs::parse();
        if let Some(config_path) = args.config {
            let manifest = Self::load_manifest_file(&config_path).with_context(|| {
                format!("Tried loading manifest from: {}. Did not.", config_path)
            })?;
            let merged: Manifest = serde_merge::tmerge(manifest, Manifest::default())?;
            return Ok(merged);
        }

        return Ok(Manifest::default());
    }

    fn load_manifest_file(path: &String) -> anyhow::Result<Manifest> {
        let loaded_config = Self::load_file(path)?;
        let config: Manifest = serde_yaml::from_str(&loaded_config)?;
        return Ok(config);
    }

    fn load_file(path: &String) -> anyhow::Result<String> {
        let file = fs_extra::file::read_to_string(&path)?;
        Ok(file)
    }
}
