use indicatif::ProgressBar;

use crate::manifest::Manifest;

use super::{playbook::playbook_builder::Playbook, task_builders::AnsibleTasks};

pub struct Ansible;

impl Ansible {
    pub fn init_templates(manifest: Manifest, spinner: &ProgressBar) {
        let this_tag = "init";

        spinner.set_prefix("🚀");
        spinner.set_message("Lumberstack launching...");

        let in_tags = manifest
            .clone()
            .tags
            .unwrap_or(Vec::new())
            .contains(&this_tag.to_string());

        if in_tags || manifest.tags.is_none() {
            Playbook::new()
                .add_task(AnsibleTasks::register_template_dir(manifest.clone()))
                .add_task(AnsibleTasks::clone_template_repo(manifest.clone()))
                .add_task(AnsibleTasks::exclude_dirs_from_search(manifest.clone()))
                .add_task(AnsibleTasks::filter_dirs())
                .add_task(AnsibleTasks::gather_template_paths())
                .add_task(AnsibleTasks::save_found_as_fact())
                .add_task(AnsibleTasks::write_template_paths_to_file(manifest.clone()))
                .run();
        }
        spinner.set_message("Initialized templates!");

    }
}
