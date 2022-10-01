// use crate::{CORE_PLAYBOOK_NAME, LUMBERSTACK_WORKDIR, app_config::AppConfig};

// use super::{playbook::{playbook_builder::{Playbook, TaskType}, yaml::command_task::{CommandTask, CommandArgs}}, write_playbook, run_playbook};

// pub fn run_core_playbook(app_config: &AppConfig) {
//     let playbook_yml = create_core_playbook_yml(app_config);
//     let core_playbook_path = format!("{}/{}", LUMBERSTACK_WORKDIR, CORE_PLAYBOOK_NAME);
//     write_playbook(&core_playbook_path, playbook_yml);
//     run_playbook(core_playbook_path);
// }

// fn create_core_playbook_yml(app_config: &AppConfig) -> Vec<Playbook> {
//     let app_name = &app_config.app_name;
//     let mut tasks: Vec<TaskType> = Vec::new();
//     let create_redwood = CommandTask {
//         name: "Creating redwood app".to_string(),
//         command: format!(
//             "yarn create redwood-app {} --typescript --overwrite",
//             &app_name
//         ),
//         args: None,
//     };
//     let generate_home_page = CommandTask {
//         name: "Generating Redwood home page".to_string(),
//         command: "yarn rw generate page home / --force".to_string(),
//         args: Some(CommandArgs {
//             chdir: format!("./{}", app_name),
//         }),
//     };

//     tasks.push(TaskType::Command(create_redwood));
//     tasks.push(TaskType::Command(generate_home_page));

//     let playbook_yml = vec![Playbook {
//         hosts: "localhost".to_string(),
//         connection: "local".to_string(),
//         tasks,
//     }];
//     return playbook_yml;
// }

use indicatif::ProgressBar;

use crate::manifest::Manifest;
use std::env;

use super::{playbook::playbook_builder::Playbook, task_builders::AnsibleTasks};

pub struct Ansible;

impl Ansible {
    pub fn init_templates(manifest: &Manifest, spinner: &ProgressBar) {
        spinner.set_prefix("🚀");
        spinner.set_message("Lumberstack launching...");

        env::set_var("ANSIBLE_NOCOWS", "True");
        env::set_var("ANSIBLE_ANY_ERRORS_FATAL", "True");
        env::set_var("ANSIBLE_LOG_PATH", &manifest.log_file);
        env::set_var("ANSIBLE_LOCALHOST_WARNING", "False");

        Playbook::new()
            .add_task(AnsibleTasks::register_template_dir(manifest))
            .add_task(AnsibleTasks::clone_template_repo(manifest))
            .add_task(AnsibleTasks::exclude_dirs_from_search(manifest))
            .add_task(AnsibleTasks::filter_dirs())
            .add_task(AnsibleTasks::gather_template_paths())
            .add_task(AnsibleTasks::save_found_as_fact())
            .add_task(AnsibleTasks::write_template_paths_to_file(manifest))
            .run();
    }
}
