use crate::{
    logger::log_skip,
    manifest::Manifest,
    task_definitions::{
        ansible::{ansible_task::RunnableAnsibleTask, yaml::command_task::CommandTask},
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag},
    },
};

#[derive(Clone)]
pub struct RedwoodApp;

impl RedwoodApp {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<RunnableAnsibleTask> {
        let app_name = &manifest.app_name.to_owned().unwrap_or_default();

        if !should_task_run(&tag, &manifest) {
            log_skip(&tag.to_string());
            return None;
        }

        let create_task = Self::create_redwood_command(&tag, app_name);
        let base_playbook = RunnableAnsibleTask::new("create redwood app").add_task(create_task);

        return Some(base_playbook);
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
