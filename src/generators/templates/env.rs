use crate::{
    cli::app_paths::{AppPaths, TemplatePaths},
    system::{utils::HandlebarBuilder, error::AppError},
};
use rand::{distributions::Alphanumeric, Rng};

pub struct AppEnv;

impl AppEnv {
    pub fn copy_template() -> Result<(), AppError> {
        let template = HandlebarBuilder::new(
            TemplatePaths::root(Some("redwood/.env.template")),
            AppPaths::root(Some(".env")),
        );
        let var_map = serde_json::json!({
            "session_secret": Self::generate_secret(),
            "database_url": format!(
            "postgresql://postgres:development@localhost:5432/{}_dev",
            AppPaths::root(None)
        ),
            "test_database_url": format!(
            "postgresql://postgres:development@localhost:5433/{}_test",
            AppPaths::root(None)
        )
        });
        template.create(Some(var_map))
    }

    fn generate_secret() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }
}
