use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{ansible_task::RunnableAnsibleTask, yaml::command_task::CommandTask},
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag},
    },
};

pub struct RedwoodAuth;

impl RedwoodAuth {
    pub fn generate_auth(tag: TaskTag, app_config: &AppConfig) -> Option<RunnableAnsibleTask> {
        let app_name = &app_config.app_name;

        if !should_task_run(&tag, app_config) {
            log_task_skip(&tag.to_string());
            return None;
        }

        let setup_auth_task = Self::setup_auth(&tag, app_name);
        let generate_secret = Self::generate_secret(&tag, app_name);
        let mut binding = RunnableAnsibleTask::new("Generating db auth");
        let core_playbook = binding.add_task(setup_auth_task).add_task(generate_secret);

        if should_task_run(&TaskTag::Pages, app_config) {
            let generate_auth_pages_task = Self::generate_auth_pages(&tag, app_name);
            core_playbook.add_task(generate_auth_pages_task);
        }

        Some(core_playbook.to_owned())
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
