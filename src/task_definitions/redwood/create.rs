use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{
            ansible_task::RunnableAnsibleTask,
            yaml::{command_task::CommandTask, debug_task::DebugTask},
        },
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag},
    },
};

#[derive(Clone)]
pub struct RedwoodApp;

impl RedwoodApp {
    pub fn create_redwood_app(tag: TaskTag, app_config: &AppConfig) -> Option<RunnableAnsibleTask> {
        let app_name = &app_config.app_name;

        if !should_task_run(&tag, app_config) {
            log_task_skip(&tag.to_string());
            return None;
        }

        let create_task = Self::create_redwood_command(&tag, app_name);
        let create_debug_task = DebugTask::create("redwood_create");
        let extra_modules_task = Self::install_extra_modules_command(&tag, app_name);
        let extra_modules_debug_task = DebugTask::create("redwood_extra_modules");

        let mut redwood_playbook = RunnableAnsibleTask::new("create redwood app");

        redwood_playbook.add_task(create_task);
        redwood_playbook.add_task(create_debug_task);
        redwood_playbook.add_task(extra_modules_task);
        redwood_playbook.add_task(extra_modules_debug_task);

        Some(redwood_playbook.to_owned())
    }

    fn create_redwood_command(tag: &TaskTag, app_name: &String) -> DefinedTask {
        let command = format!(
            "yarn create redwood-app {} --typescript --overwrite",
            app_name
        );
        CommandTask::new("create redwood app")
            .set_tags(Some(vec![tag.to_string()]))
            .command(command)
            .register("redwood_create")
            .build()
    }

    fn install_extra_modules_command(tag: &TaskTag, app_name: &str) -> DefinedTask {
        CommandTask::new("Extra modules")
            .set_tags(Some(vec!["modules".to_string(), tag.to_string()]))
            .command("yarn add -D chance")
            .chdir(app_name)
            .register("redwood_extra_modules")
            .build()
    }
}
