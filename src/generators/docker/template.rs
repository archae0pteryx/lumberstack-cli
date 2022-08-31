use crate::{
    cli::{
        app_paths::{AppPaths, TemplatePaths},
        progress::AppProgress,
    },
    system::{error::AppError, utils::HandlebarBuilder},
};

pub fn create_compose(progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update("🐳 Creating docker compose");
    let source = TemplatePaths::root(Some("redwood/docker-compose.template"));
    let destination = AppPaths::root(Some("docker-compose.yml"));
    let template = HandlebarBuilder::new(source, destination);
    let replace_map = serde_json::json!({ "app_name": AppPaths::root(None) });
    template.create(Some(replace_map))
}
