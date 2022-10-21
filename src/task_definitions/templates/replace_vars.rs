use std::collections::HashMap;

use crate::app_config::AppConfig;

use super::symbols::Symbols;

pub struct Replacer;

impl Replacer {
    pub fn process_and_replace_vars(file_str: &str, app_config: AppConfig) -> String {
        let symbol_replace_vars = Symbols::get_replacers(file_str);
        Self::replace_in_content(&app_config, symbol_replace_vars, file_str)
    }

    fn replace_in_content(
        app_config: &AppConfig,
        symbol_vars: HashMap<String, String>,
        content: &str,
    ) -> String {
        let mut replaced_content = content.to_string();
        let interpolated_vars = Self::interpolate_vars(app_config, symbol_vars);
        for (key, value) in interpolated_vars {
            replaced_content = replaced_content.replace(&key, &value);
        }
        replaced_content
    }

    fn interpolate_vars(
        app_config: &AppConfig,
        symbol_vars: HashMap<String, String>,
    ) -> HashMap<String, String> {
        let mut interpolated_vars: HashMap<String, String> = HashMap::new();

        for (key, value) in &app_config.replace_vars {
            let has_val = symbol_vars.contains_key(key);
            if !has_val {
                interpolated_vars.insert(key.to_string(), value.to_string());
            }
        }

        for (key, value) in symbol_vars {
            if value.starts_with('$') {
                let v = Self::clip_dollar_sign(&value);
                let config_var = interpolated_vars.get(&v);

                if let Some(config_var) = config_var {
                    interpolated_vars.insert(key, config_var.to_owned());
                } else {
                    panic!("No config var found for {}", v);
                }
            } else {
                interpolated_vars.insert(key.to_string(), value.to_string());
            }
        }
        let sanity_check = interpolated_vars.get("app_name").unwrap();
        assert_eq!(sanity_check, &app_config.app_name);
        interpolated_vars
    }

    fn clip_dollar_sign(value: &String) -> String {
        if value.starts_with('$') {
            return value.trim_start_matches('$').to_string();
        }
        value.to_owned()
    }
}
