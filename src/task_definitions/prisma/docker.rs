use crate::{
    app_config::AppConfig,
    task_definitions::{ansible::yaml::command_task::CommandTask, task_types::DefinedTask},
};

pub struct Docker;

impl Docker {
    pub fn compose_start(app_config: &AppConfig) -> DefinedTask {
        CommandTask::new("Docker Compose Up")
            .command("docker-compose up -d")
            .chdir(&app_config.app_name)
            .set_tags(Some(vec!["docker".to_string(), "prisma".to_string()]))
            .register("docker_compose_start")
            .build()
    }

    pub fn reset(app_config: &AppConfig) -> DefinedTask {
        CommandTask::new("Removing old docker containers")
            .command("docker-compose down -v")
            .chdir(&app_config.app_name)
            .set_tags(Some(vec!["docker".to_string(), "prisma".to_string()]))
            .register("docker_reset")
            .build()
    }
}
