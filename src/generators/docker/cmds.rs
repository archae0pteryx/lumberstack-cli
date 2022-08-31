#![allow(unused)]
use crate::{
    cli::{app_paths::AppPaths, progress::AppProgress},
    system::{error::AppError, shell::Shell},
};

pub(super) fn up_db(database: &str, progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update(format!("🐳 Starting {}", database).as_str());
    let compose_file = AppPaths::root(Some("docker-compose.yml"));
    Shell::exec(format!(
        "docker compose -f {} up {} -d",
        compose_file, database
    ))?;
    Ok(())
}

pub(super) fn down_db(database: &str, progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update(format!("🐳 Stopping {}", database).as_str());
    let compose_file = AppPaths::root(Some("docker-compose.yml"));
    cmd_lib::run_fun!(docker compose -f $compose_file stop db)?;
    Shell::exec(format!(
        "docker compose -f {} stop {}",
        compose_file, database
    ))?;
    Ok(())
}

pub(super) fn remove_volume(database: &str, progress_bar: &AppProgress) -> Result<(), AppError> {
    progress_bar.update(format!("🐳 Removing volume for {}", database).as_str());
    let compose_file = AppPaths::root(Some("docker-compose.yml"));
    Shell::exec(format!(
        "docker compose -f {} rm {} -v --force",
        compose_file, database
    ))?;
    Ok(())
}
