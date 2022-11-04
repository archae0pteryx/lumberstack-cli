use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{
            ansible_task::RunnableAnsibleTask,
            yaml::{command_task::CommandTask, debug_task::DebugTask},
        },
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag, tag_to_str},
    },
};

pub struct Playwright;

impl Playwright {
    pub fn create_playwright(tag: TaskTag, app_config: &AppConfig) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag);
            return None;
        }
        let app_name = app_config.app_name.to_owned();
        let mut playbook = RunnableAnsibleTask::new("Playwright");

        let playwright_task = Self::install_command(&tag, &app_name);
        let playwright_debug_task = DebugTask::create("playwright_install");

        playbook.add_task(playwright_task);
        playbook.add_task(playwright_debug_task);

        Some(playbook)
    }

    fn install_command(tag: &TaskTag, app_name: &str) -> DefinedTask {
        let web_dir = format!("{}/web", app_name);
        CommandTask::new("Setup playwright")
            .chdir(web_dir)
            .set_tags(&vec!["playwright".to_string(), tag_to_str(tag)])
            .command("yarn create playwright --quiet --lang=ts")
            .register("playwright_install")
            .build()
    }
}
