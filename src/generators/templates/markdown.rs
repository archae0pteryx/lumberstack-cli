use super::{FileMap, Templates};
use crate::{cli::app_paths::{AppPaths, TemplatePaths}, system::error::AppError};
use chrono::{Datelike, Utc};

pub struct Markdown;

impl Markdown {
    pub fn copy_templates() -> Result<(), AppError> {
        let contrib_md = FileMap {
            source: TemplatePaths::root(Some("md/CONTRIBUTING.md.template")),
            destination: AppPaths::root(Some("CONTRIBUTING.md")),
            replace_map: None,
        };
        let license_md = FileMap {
            source: TemplatePaths::root(Some("md/LICENSE.md.template")),
            destination: AppPaths::root(Some("LICENSE.md")),
            replace_map: Some(serde_json::json!({
                "year": Self::get_year(),
                "app_name": AppPaths::root(None)
            })),
        };

        let readme_md = FileMap {
            source: TemplatePaths::root(Some("md/README.md.template")),
            destination: AppPaths::root(Some("README.md")),
            replace_map: None,
        };

        let data = vec![contrib_md, license_md, readme_md];
        Templates::replace_and_load(data)
    }

    fn get_year() -> i32 {
        let current_date = Utc::now();
        current_date.year()
    }
}
