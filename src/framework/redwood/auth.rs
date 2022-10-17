use crate::{
    logger::log_skip,
    manifest::Manifest,
    task_definitions::{
        ansible::{ansible_task::RunnableAnsibleTask, yaml::command_task::CommandTask},
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag},
    },
};

pub struct RedwoodAuth;

impl RedwoodAuth {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<RunnableAnsibleTask> {
        let app_name = manifest.app_name.to_owned().unwrap_or_default();

        if !should_task_run(&tag, &manifest) {
            log_skip(&tag.to_string());
            return None;
        }

        let setup_auth_task = Self::setup_auth(&tag, &app_name);
        let generate_secret = Self::generate_secret(&tag, &app_name);
        let core_playbook = RunnableAnsibleTask::new("Generating db auth")
            .add_task(setup_auth_task)
            .add_task(generate_secret);

        if should_task_run(&TaskTag::Pages, &manifest) {
            let generate_auth_pages_task = Self::generate_auth_pages(&tag, &app_name);
            core_playbook.add_task(generate_auth_pages_task);
        }

        return Some(core_playbook);
    }

    fn setup_auth(tag: &TaskTag, app_name: &String) -> DefinedTask {
        CommandTask::new("Setting up auth")
            .chdir(app_name)
            .command("yarn rw setup auth dbAuth --force")
            .set_tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn generate_auth_pages(tag: &TaskTag, app_name: &String) -> DefinedTask {
        CommandTask::new("Generating auth pages")
            .chdir(app_name)
            .command("yarn rw generate dbAuth --force")
            .set_tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn generate_secret(tag: &TaskTag, app_name: &String) -> DefinedTask {
        CommandTask::new("Generating session secret")
            .chdir(app_name)
            .command("yarn rw generate secret")
            .set_tags(Some(vec![tag.to_string()]))
            .build()
    }
}