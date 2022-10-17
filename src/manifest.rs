// use std::{collections::HashMap, env};

// use anyhow::{Context, Error, Result};
// use clap::Parser;
// use log::debug;
// use serde::{Deserialize, Serialize};

// use crate::{
//     app_config::{
//         DEFAULT_APP_NAME, DEFAULT_MANIFEST_FILE, DEFAULT_TEMPLATE_DIR, DEFAULT_TEMPLATE_PATHS_FILE,
//         DEFAULT_TEMPLATE_REPO, DEFAULT_TEMPLATE_VERSION, DEFAULT_WORKDIR,
//     },
//     cli_args::CliArgs,
// };

// pub fn default_var_replace_map() -> HashMap<String, String> {
//     let mut m = HashMap::new();
//     m.insert("$app_name".to_string(), DEFAULT_APP_NAME.to_string());
//     m
// }

// /// The core config file for the CLI
// /// It is created by both CLI arguments as well as a config file
// /// Arguments always take precidence over the file
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Manifest {
//     /// The project name and destination
//     pub app_name: Option<String>,
//     /// The repository for the template app to use
//     pub template_repo: Option<String>,
//     /// Which github release to pull
//     pub template_version: Option<String>,
//     /// Remove all artifacts from previous builds
//     pub clean: Option<bool>,
//     /// The root dir for work to occur
//     pub workdir: Option<String>,
//     /// The directory name to clone templates into
//     pub template_dir: Option<String>,
//     /// The combined workdir and template_dir
//     pub full_template_path: Option<String>,
//     /// The file to save a list of templates to do work with
//     pub template_paths_file: Option<String>,
//     /// Extra logging (ansible throughput)
//     pub log_file: Option<String>,
//     /// The tags to run individual tasks
//     pub tags: Option<Vec<String>>,
//     /// Tags to skip (the reverse of tags)
//     pub skip_tags: Option<Vec<String>>,
//     /// A map of key values to replace in the templates
//     /// Rules for replacement are defined in the templates themselves
//     pub replace: Option<HashMap<String, String>>,
// }
// trait Empty<T> {
//     fn empty() -> T;
// }

// impl Empty<Manifest> for Manifest {
//     fn empty() -> Manifest {
//         Manifest {
//             app_name: None,
//             template_version: None,
//             workdir: None,
//             clean: None,
//             template_repo: None,
//             template_dir: None,
//             full_template_path: None,
//             template_paths_file: None,
//             log_file: None,
//             tags: None,
//             skip_tags: None,
//             replace: None,
//         }
//     }
// }

// impl Default for Manifest {
//     fn default() -> Self {
//         let replacers = default_var_replace_map();
//         Manifest {
//             app_name: Some(DEFAULT_APP_NAME.to_string()),
//             template_version: Some(DEFAULT_TEMPLATE_VERSION.to_string()),
//             clean: Some(true),
//             workdir: Some(DEFAULT_WORKDIR.to_string()),
//             template_repo: Some(DEFAULT_TEMPLATE_REPO.to_string()),
//             template_dir: Some(DEFAULT_TEMPLATE_DIR.to_string()),
//             full_template_path: Some(format!(
//                 "{}/{}",
//                 DEFAULT_WORKDIR.to_string(),
//                 DEFAULT_TEMPLATE_DIR.to_string()
//             )),
//             template_paths_file: Some(DEFAULT_TEMPLATE_PATHS_FILE.to_string()),
//             log_file: None,
//             tags: None,
//             skip_tags: None,
//             replace: Some(replacers),
//         }
//     }
// }

// impl Manifest {
//     pub fn load() -> Result<Manifest> {
//         let args = CliArgs::parse();
//         let config_manifest = Self::config_manifest(args.config)?;

//         let arg_manifest = Self::args_manifest();

//         let user_item_manifest: Manifest = serde_merge::omerge(config_manifest, arg_manifest)?;

//         let merged_manifest: Manifest =
//             serde_merge::omerge(Manifest::default(), user_item_manifest)?;

//         // dbg!(&merged_manifest);

//         Self::set_env(&merged_manifest);
//         Self::manifest_debug_log(&merged_manifest);
//         return Ok(merged_manifest);
//     }

//     fn config_manifest(path: Option<String>) -> Result<Manifest> {
//         if let Some(p) = path {
//             debug!("[manifest] from cli arg");
//             let loaded_config = Self::load_file(&p)?;
//             let config: Manifest = Self::deserialize_config(loaded_config)?;
//             return Ok(config);
//         }
//         if let Ok(c) = Self::load_file(&DEFAULT_MANIFEST_FILE.to_string()) {
//             debug!("[manifest] from default location");
//             let config: Manifest = Self::deserialize_config(c)?;
//             return Ok(config);
//         }
//         debug!("[manifest] from empty config");
//         Ok(Manifest::empty())
//     }

//     fn deserialize_config(loaded_config: String) -> Result<Manifest> {
//         let from_yml: Result<Manifest, Error> = serde_yaml::from_str(&loaded_config)
//             .with_context(|| "[manifest] Error deserializing loaded yaml".to_string());
//         if let Ok(m) = from_yml {
//             return Ok(m);
//         }

//         let from_json: Result<Manifest, Error> = serde_json::from_str(&loaded_config)
//             .with_context(|| "[manifest] Error deserializing loaded json".to_string());

//         if let Ok(m) = from_json {
//             return Ok(m);
//         }

//         return Ok(Manifest::default());
//     }

//     fn load_file(path: &String) -> Result<String> {
//         let file = fs_extra::file::read_to_string(&path)
//             .with_context(|| format!("[manifest] Tried to load: {} but could not", path))?;
//         Ok(file)
//     }

//     fn args_manifest() -> Manifest {
//         let args = CliArgs::parse();

//         Manifest {
//             app_name: args.name,
//             template_version: args.template_version,
//             tags: args.tags,
//             skip_tags: args.skip_tags,
//             log_file: args.log_file,
//             ..Manifest::empty()
//         }
//     }

//     fn set_env(manifest: &Manifest) {
//         env::set_var("ANSIBLE_NOCOWS", "True");
//         env::set_var("ANSIBLE_ANY_ERRORS_FATAL", "True");
//         env::set_var("ANSIBLE_LOCALHOST_WARNING", "False");
//         Self::set_logger(manifest);
//     }

//     fn set_logger(manifest: &Manifest) {
//         if let Some(log_path) = &manifest.log_file {
//             env::set_var("ANSIBLE_LOG_PATH", log_path);
//         }
//     }

//     fn manifest_debug_log(manifest: &Manifest) {
//         let app_name = manifest.app_name.to_owned();
//         let template_version = manifest.template_version.to_owned();
//         let tags = manifest.tags.to_owned();
//         let skip_tags = manifest.skip_tags.to_owned();
//         let msg = format!(
//             "[manifest]: app_name=[{}] template_version=[{}] run=[{:?}] skip=[{:?}]",
//             &app_name.unwrap(),
//             &template_version.unwrap(),
//             &tags.unwrap_or_default(),
//             &skip_tags.unwrap_or_default()
//         );
//         debug!("{}", msg);
//     }
// }
