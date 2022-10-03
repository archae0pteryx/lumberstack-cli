use std::env;

use anyhow::{Context, Error, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    cli_args::CliArgs, DEFAULT_APP_NAME, DEFAULT_TEMPLATE_DIR, DEFAULT_TEMPLATE_PATHS_FILE,
    DEFAULT_TEMPLATE_REPO, DEFAULT_TEMPLATE_VERSION, DEFAULT_WORKDIR,
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
trait Empty<T> {
    fn empty() -> T;
}

impl Empty<Manifest> for Manifest {
    fn empty() -> Manifest {
        Manifest {
            app_name: None,
            template_version: None,
            workdir: None,
            clean: None,
            template_repo: None,
            template_dir: None,
            template_paths_file: None,
            log_file: None,
            tags: None,
        }
    }
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
            log_file: None,
            tags: None,
        }
    }
}

impl Manifest {
    pub fn load() -> Result<Manifest> {
        let args = CliArgs::parse();
        let config_manifest = Self::config_manifest(args.config)?;

        let arg_manifest = Self::args_manifest();

        let user_item_manifest: Manifest = serde_merge::omerge(config_manifest, arg_manifest)?;

        let merged_manifest: Manifest =
            serde_merge::omerge(Manifest::default(), user_item_manifest)?;

        // dbg!(&merged_manifest);

        Self::set_env(&merged_manifest);
        return Ok(merged_manifest);
    }

    fn config_manifest(path: Option<String>) -> Result<Manifest> {
        if let Some(p) = path {
            let loaded_config = Self::load_file(&p)?;
            let config: Manifest = Self::deserialize_config(loaded_config)?;
            return Ok(config);
        }
        Ok(Manifest::empty())
    }

    fn deserialize_config(loaded_config: String) -> Result<Manifest> {
        let from_yml: Result<Manifest, Error> = serde_yaml::from_str(&loaded_config)
            .with_context(|| "Error deserializing loaded manifest yaml".to_string());
        if let Ok(m) = from_yml {
            return Ok(m);
        }

        let from_json: Result<Manifest, Error> = serde_json::from_str(&loaded_config)
            .with_context(|| "Error deserializing loaded manifest json".to_string());

        if let Ok(m) = from_json {
            return Ok(m);
        }

        return Ok(Manifest::default());
    }

    fn load_file(path: &String) -> Result<String> {
        let file = fs_extra::file::read_to_string(&path)
            .with_context(|| format!("Tried to load: {} but could not", path))?;
        Ok(file)
    }

    fn args_manifest() -> Manifest {
        let args = CliArgs::parse();

        Manifest {
            app_name: args.name,
            template_version: args.template_version,
            tags: args.tags,
            ..Manifest::empty()
        }
    }

    fn set_env(manifest: &Manifest) {
        env::set_var("ANSIBLE_NOCOWS", "True");
        env::set_var("ANSIBLE_ANY_ERRORS_FATAL", "True");
        env::set_var("ANSIBLE_LOCALHOST_WARNING", "False");
        Self::set_logger(manifest);
    }

    fn set_logger(manifest: &Manifest) {
        if let Some(log_path) = &manifest.log_file {
            let lp = format!("{}/{}", DEFAULT_WORKDIR, log_path);
            env::set_var("ANSIBLE_LOG_PATH", lp);
        }
    }
}
