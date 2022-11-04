use log::info;

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

pub struct Heroku;

impl Heroku {
    pub fn create_heroku(tag: TaskTag, app_config: &AppConfig) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag);
            return None;
        }
        let app_name = app_config.app_name.to_owned();
        let mut playbook = RunnableAnsibleTask::new("Heroku");

        let heroku_task = Self::install_pm2(tag, &app_name);
        let heroku_debug_task = DebugTask::create("heroku_install");

        playbook.add_task(heroku_task);
        playbook.add_task(heroku_debug_task);

        info!("Heroku deployments are not yet supported");

        Some(playbook)
    }

    fn install_pm2(tag: TaskTag, app_name: &str) -> DefinedTask {
        CommandTask::new("Install PM2")
            .chdir(app_name)
            .set_tags(&["heroku".to_string(), tag_to_str(&tag)])
            .command("yarn add pm2")
            .register("heroku_install")
            .build()
    }
}
