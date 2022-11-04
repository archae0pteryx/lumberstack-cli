use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{
            ansible_task::RunnableAnsibleTask,
            yaml::{command_task::CommandTask, debug_task::DebugTask},
        },
        task_types::DefinedTask,
        templates::tags::{should_task_run, tag_to_str, TaskTag},
    },
};

pub struct Tailwind;

impl Tailwind {
    pub fn create_tailwind(tag: TaskTag, app_config: &AppConfig) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag);
            return None;
        }
        let app_name = app_config.app_name.to_owned();
        let mut playbook = RunnableAnsibleTask::new("Tailwind");

        let tailwind_task = Self::install_command(&tag, &app_name);
        let tailwind_debug_task = DebugTask::create("tailwind_install");

        playbook.add_task(tailwind_task);
        playbook.add_task(tailwind_debug_task);

        Some(playbook)
    }

    fn install_command(tag: &TaskTag, app_name: &str) -> DefinedTask {
        CommandTask::new("Setup tailwind")
            .set_tags(&[tag_to_str(&TaskTag::Tailwind), tag_to_str(tag)])
            .command("yarn rw setup ui tailwind")
            .chdir(app_name)
            .register("tailwind_install")
            .build()
    }
}
