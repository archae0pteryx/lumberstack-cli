use std::collections::HashMap;

use clap::Error;
use handlebars::JsonValue;

use crate::system::{error::AppError, utils::load_external_json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// "name": "myapp",
// "builder": [
//   {
//     "feedback": "creating redwood app",
//     "commands": ["yarn create redwood-app {{app_name}}"],
//     "replace_map": {}
//   }
// ]

#[derive(Serialize, Deserialize, Debug)]
struct ReplaceMap {
    key: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigBuilder {
    feedback: String,
    commands: Vec<String>,
    replace_map: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigJson {
    name: String,
    builder: Vec<ConfigBuilder>,
}

pub struct TemplateConfigFile;

impl TemplateConfigFile {
    pub fn load() -> Result<Value, AppError> {
        let current_dir = std::env::current_dir();
        let default_config_path = format!(
            "{}/src/configuration/default_config.json",
            current_dir.unwrap().as_path().to_str().unwrap()
        );
        let path = format!("{}/default_config.json", default_config_path);
        let loaded = load_external_json(path.as_str());
        println!("loaded {:?}", loaded);
        loaded
    }
}

const DEFAULT_CONFIG: &str = r#"
  {
    "name": "myapp",
    "builder": [
      {
        "feedback": "creating redwood app",
        "commands": ["yarn create redwood-app {{app_name}}"],
        "replace_map": {}
      }
    ]
  }
"#;
