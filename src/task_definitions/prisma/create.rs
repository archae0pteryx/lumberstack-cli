use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{ansible_task::RunnableAnsibleTask, yaml::{command_task::CommandTask, debug_task::DebugTask}},
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag, tag_to_str},
    },
};

use super::docker::Docker;

pub struct Prisma;

impl Prisma {
    pub fn setup_prisma(
        tag: TaskTag,
        app_config: &AppConfig,
    ) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag);
            return None;
        }

        let reset_docker_task = Docker::reset(app_config);
        let reset_docker_debug = DebugTask::create("docker_reset");
        let compose_start_task = Docker::compose_start(app_config);
        let docker_compose_debug = DebugTask::create("docker_compose_start");
        let migrate_task = Self::migrate(app_config);
        let migrate_debug = DebugTask::create("prisma_migrate");
        let seed_task = Self::seed(app_config);
        let seed_debug = DebugTask::create("prisma_seed");

        let mut base_playbook = RunnableAnsibleTask::new("Setup Prisma");

        base_playbook
            .add_task(reset_docker_task)
            .add_task(reset_docker_debug)
            .add_task(compose_start_task)
            .add_task(docker_compose_debug)
            .add_task(migrate_task)
            .add_task(migrate_debug)
            .add_task(seed_task)
            .add_task(seed_debug);

        Some(base_playbook)
    }

    fn migrate(app_config: &AppConfig) -> DefinedTask {
        CommandTask::new("Migrate Prisma")
            .command("yarn redwood prisma migrate dev")
            .chdir(&app_config.app_name)
            .set_tags(&vec![tag_to_str(&TaskTag::Prisma)])
            .register("prisma_migrate")
            .build()
    }
    fn seed(app_config: &AppConfig) -> DefinedTask {
        CommandTask::new("Seed Prisma")
            .command("yarn redwood prisma db seed")
            .chdir(&app_config.app_name)
            .set_tags(&vec!["prisma".to_string()])
            .register("prisma_seed")
            .build()
    }
}
