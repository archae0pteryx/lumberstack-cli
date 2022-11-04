use crate::{
    app_config::AppConfig,
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{ansible_task::RunnableAnsibleTask, yaml::command_task::CommandTask},
        task_types::DefinedTask,
        templates::tags::{should_task_run, TaskTag},
    },
};

pub struct RedwoodGenerate;

impl RedwoodGenerate {
    pub fn generate_pages(tag: TaskTag, app_config: &AppConfig) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag);
            return None;
        }

        let mut playbook = RunnableAnsibleTask::new("Generating Pages and Layouts");

        let page_tasks = Self::gather_pages(app_config);
        for task in page_tasks {
            playbook.add_task(task.to_owned());
        }

        let layout_tasks = Self::gather_layouts(app_config);
        for task in layout_tasks {
            playbook.add_task(task.to_owned());
        }
        // dbg!(&playbook);
        Some(playbook)
    }

    fn gather_pages(app_config: &AppConfig) -> Vec<DefinedTask> {
        app_config
            .generate_pages
            .iter()
            .map(|page| Self::generate_page(&app_config.app_name, page))
            .collect::<Vec<_>>()
    }

    fn gather_layouts(app_config: &AppConfig) -> Vec<DefinedTask> {
        app_config
            .generate_layouts
            .iter()
            .cloned()
            .map(|layout| Self::generate_layout(&app_config.app_name, layout))
            .collect::<Vec<_>>()
    }

    fn generate_page(app_name: &String, (name, path): (&String, &String)) -> DefinedTask {
        let command = format!("yarn redwood generate page {} {}", &name, &path);
        CommandTask::new(format!("Generating page: {}", &name))
            .command(command)
            .chdir(app_name)
            .build()
    }

    fn generate_layout(app_name: &String, layout_name: String) -> DefinedTask {
        let command = format!("yarn redwood generate layout {} --ts", layout_name);
        CommandTask::new(format!("Generating layout: {}", &layout_name))
            .command(command)
            .chdir(app_name)
            .build()
    }
}
