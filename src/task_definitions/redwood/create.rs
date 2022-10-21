use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{ansible_task::RunnableAnsibleTask, yaml::command_task::CommandTask},
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag},
    },
};

#[derive(Clone)]
pub struct RedwoodApp;

impl RedwoodApp {
    pub fn create_redwood_app(
        tag: TaskTag,
        app_config: &AppConfig,
    ) -> Option<RunnableAnsibleTask> {
        let app_name = &app_config.app_name;

        if !should_task_run(&tag, app_config) {
            log_task_skip(&tag.to_string());
            return None;
        }

        let create_task = Self::create_redwood_command(&tag, app_name);
        let mut base_playbook = RunnableAnsibleTask::new("create redwood app");
        base_playbook.add_task(create_task);

        Some(base_playbook.to_owned())
    }

    fn create_redwood_command(tag: &TaskTag, app_name: &String) -> DefinedTask {
        let command = format!(
            "yarn create redwood-app {} --typescript --overwrite",
            app_name
        );
        CommandTask::new("create redwood app")
            .set_tags(Some(vec![tag.to_string()]))
            .command(command)
            .build()
    }
}
