use super::error::AppError;
use handlebars::Handlebars;
use log::debug;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

pub struct HandlebarBuilder<'a> {
    source: String,
    destination: String,
    handlebars: Handlebars<'a>,
}

impl HandlebarBuilder<'_> {
    pub fn new<'a>(source: String, destination: String) -> HandlebarBuilder<'a> {
        let mut handlebars = Handlebars::new();
        let tpl_str = fs::read_to_string(&source)
            .map_err(|err| AppError {
                message: format!(
                    "[hb::new] Error reading source file: [{}], err: {}",
                    source,
                    err.to_string()
                ),
            })
            .unwrap();
        let loaded_template = handlebars.register_template_string(&source, tpl_str);
        assert!(loaded_template.is_ok());
        HandlebarBuilder {
            source: String::from(source),
            destination: String::from(destination),
            handlebars: handlebars,
        }
    }

    pub fn create(&self, replace_map: Option<Value>) -> Result<(), AppError> {
        return match replace_map {
            Some(value) => {
                let file_with_replaced_data = self.replace(value);
                self.create_destination_file(file_with_replaced_data)
            }
            None => {
                let rendered = self.replace(serde_json::json!({}));
                self.create_destination_file(rendered)
            }
        };
    }

    fn create_destination_file(&self, data: String) -> Result<(), AppError> {
        let mut file = fs::File::create(&self.destination)?;
        file.write_all(data.as_bytes()).map_err(|err| AppError {
            message: format!(
                "[utils::create_destination_file] ❌ Error writing file {} - {}",
                &self.destination, err
            ),
        })?;
        Ok(())
    }

    fn replace(&self, data: Value) -> String {
        let mut btree_map = BTreeMap::new();
        for (key, value) in data.as_object().unwrap() {
            btree_map.insert(key, value);
        }
        self.handlebars
            .render(&self.source, &data)
            .map_err(|err| AppError {
                message: format!(
                    "[utils::replace] ❌ Error rendering template data: {}",
                    err.to_string()
                ),
            })
            .unwrap()
    }
}

pub fn load_external_json<P: AsRef<Path>>(path: P) -> Result<Value, AppError> {
    debug!(
        "🔦 [utils::load_external_json] loading external json from {}",
        path.as_ref().to_string_lossy()
    );
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let loaded_json = serde_json::from_reader(reader).map_err(|_| AppError {
        message: "[utils::load_external_json] ❌ Error loading JSON".to_string(),
    })?;
    Ok(loaded_json)
}

pub fn load_template_json(path: &str) -> Result<Value, AppError> {
    debug!("🔦 [utils::load_template_json] loading template json from {}", path);
    let json_str = fs::read_to_string(path)?;
    let loaded_json: Value = serde_json::from_str(json_str.as_str())?;
    Ok(loaded_json)
}

pub fn merge_json(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_json(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub fn copy_file(source: String, dest: String) -> Result<(), AppError> {
    debug!(
        "🔦 [utils::copy_file] copying file from {} to {}",
        source, dest
    );
    let mut options = fs_extra::file::CopyOptions::new();
    options.overwrite = true;
    fs_extra::file::copy(&source, &dest, &options).map_err(|err| AppError {
        message: format!(
            "[utils::copy_file] ❌ Error copying file [{}] to [{}]. Error: {}",
            source, dest, err
        ),
    })?;
    Ok(())
}

pub fn copy_dir(source: String, dest: String) -> Result<(), AppError> {
    debug!(
        "🔦 [utils::copy_dir] copying dir from {} to {}",
        source, dest
    );
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    fs_extra::dir::copy(&source, &dest, &options).map_err(|err| AppError {
        message: format!(
            "[utils::copy_dir] ❌ Error copying dir [{}] to [{}]. Error: {}",
            source, dest, err
        ),
    })?;
    Ok(())
}
