use serde_json::json;
use std::fs;

use crate::{
    cli::{
        app_paths::{AppPaths, TemplatePaths},
        progress::AppProgress,
    },
    generators::templates::{FileMap, Templates}, system::error::AppError,
};

pub(super) fn copy_heroku_templates(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("Copying Heroku templates");

    fs::create_dir_all(AppPaths::root(Some("config")))?;

    let app_json = FileMap {
        source: TemplatePaths::root(Some("heroku/app.json.template")),
        destination: AppPaths::root(Some("app.json")),
        replace_map: None,
    };

    let ngnix_config = FileMap {
        source: TemplatePaths::root(Some("heroku/nginx.config.erb.template")),
        destination: AppPaths::root(Some("config/nginx.config.erb")),
        replace_map: None,
    };

    let pm2_index = FileMap {
        source: TemplatePaths::root(Some("heroku/pm2.index.js.template")),
        destination: AppPaths::root(Some("index.js")),
        replace_map: Some(json!({ "app_name": AppPaths::root(None) })),
    };

    let procfile = FileMap {
        source: TemplatePaths::root(Some("heroku/Procfile.template")),
        destination: AppPaths::root(Some("Procfile")),
        replace_map: None,
    };

    let data = vec![app_json, ngnix_config, pm2_index, procfile];
    Templates::replace_and_load(data)
}
