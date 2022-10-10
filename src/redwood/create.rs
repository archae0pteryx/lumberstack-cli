use crate::{
    ansible::playbook::{
        yaml::{command_task::CommandTask, task_type::PlaybookYamlTaskType}, create::Playbook,
    },
    manifest::Manifest,
    tags::{should_task_run, TaskTag},
};

#[derive(Clone)]
pub struct RedwoodApp;

impl RedwoodApp {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<Playbook> {
        let tags = &manifest.tags.to_owned();
        let app_name = &manifest.app_name.to_owned().unwrap_or_default();
        if !should_task_run(&tag, &tags) {
            return None;
        }

        let create_task = Self::create_redwood_command(&tag, app_name);
        let base_playbook = Playbook::new("create redwood app").add_task(create_task);

        return Some(base_playbook);
    }

    fn create_redwood_command(tag: &TaskTag, app_name: &String) -> PlaybookYamlTaskType {
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
