use super::error::AppError;
use crate::cli::logger::Logger;
use handlebars::Handlebars;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    io::{BufReader, Write},
    path::Path,
};

pub struct HandlebarBuilder<'a> {
    source: String,
    destination: String,
    handlebars: Handlebars<'a>,
}

impl HandlebarBuilder<'_> {
    pub fn new<'a>(source: String, destination: String) -> HandlebarBuilder<'a> {
        let mut handlebars = Handlebars::new();
        let tpl_str = Self::load_template(&source);
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
            message: format!("❌ Error writing file {} - {}", &self.destination, err),
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
                message: format!("❌ Error rendering template data: {}", err.to_string()),
            })
            .unwrap()
    }

    fn load_template(source_file: &str) -> String {
        let contents = fs::read_to_string(source_file).map_err(|err| AppError {
            message: format!("❌ Error loading template {source_file}: {}", err.to_string()),
        });
        contents.unwrap()
    }
}

pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Value, AppError> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let loaded_json = serde_json::from_reader(reader).map_err(|_| AppError {
        message: "❌ Error loading JSON".to_string(),
    })?;
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
    Logger::loud_info(format!("📄 Copying file {} -> {}", &source, &dest));
    fs::copy(&source, &dest).map_err(|_| AppError {
        message: format!("❌ Error copying file: {}", &source),
    })?;
    Ok(())
}

pub fn copy_directory(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), AppError> {
    Logger::loud_info(format!(
        "🗄 Copying dir {:?} -> {:?}",
        source.as_ref().as_os_str(),
        dest.as_ref().as_os_str()
    ));
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(&source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name())).map_err(|_| {
                AppError {
                    message: format!(
                        "Error copying dir {} -> {}",
                        &source.as_ref().to_string_lossy(),
                        &dest.as_ref().to_string_lossy()
                    ),
                }
            })?;
        }
    }
    Ok(())
}
