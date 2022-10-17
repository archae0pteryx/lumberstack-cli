use crate::{
    logger::log_skip,
    manifest::Manifest,
    task_definitions::{
        ansible::{
            ansible_task::RunnableAnsibleTask,
            yaml::{
                copy_task::CopyTask, fact_task::FactTask, find_task::FindTask, git_task::GitTask,
                register_task::RegisterTask,
            },
        },
        task_types::DefinedTask,
    }, app_config::DEFAULT_ANSIBLE_TEMPLATE_REGEX,
};

use super::tags::{should_task_run, TaskTag};

#[derive(Clone)]

pub struct TemplatesClone;

impl TemplatesClone {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, &manifest) {
            log_skip(tag.to_string());
            return None;
        }
        // Register a dir for ansible to manipulate
        let register_task = Self::register_template_dir(tag.clone(), manifest.clone());
        // Clone the template repo
        let clone_task = Self::clone_template_repo(tag.clone(), manifest.clone());
        // Exclude unnecessary dirs we search through and filter
        let exclude_dirs_task = Self::exclude_dirs_from_search(tag.clone(), manifest.clone());
        // Filter the directories
        let filter_task = Self::filter_dirs(tag.clone());
        // After we filter we gather all the matching file paths for templates
        let gather_paths_task = Self::gather_template_paths(tag.clone());
        // Save the matching template paths as a var
        let save_fact_task = Self::save_found_as_fact(tag.clone());
        // Write the paths var to a file to be read by rust
        let write_paths_task = Self::write_template_paths_to_file(tag.clone(), manifest.clone());
        // The playbook combines them all together into a playbook that we can execute
        Some(
            RunnableAnsibleTask::new("Cloning Templates")
                .add_task(register_task)
                .add_task(clone_task)
                .add_task(exclude_dirs_task)
                .add_task(filter_task)
                .add_task(gather_paths_task)
                .add_task(save_fact_task)
                .add_task(write_paths_task),
        )
    }

    fn register_template_dir(tag: TaskTag, manifest: Manifest) -> DefinedTask {
        RegisterTask::new("Register template dir")
            .register("tmp_templates")
            .stat_path(manifest.full_template_path.clone().unwrap_or_default())
            .tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn clone_template_repo(tag: TaskTag, manifest: Manifest) -> DefinedTask {
        let repo = manifest.template_repo;
        let ver = manifest.template_version;
        let template_path = manifest.full_template_path;

        GitTask::new("Clone template repo")
            .repo(repo.unwrap_or_default())
            .dest(template_path.unwrap_or_default())
            .version(ver.unwrap_or_default())
            .when("not tmp_templates.stat.exists")
            .tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn exclude_dirs_from_search(tag: TaskTag, manifest: Manifest) -> DefinedTask {
        let workdir = manifest.workdir;
        FindTask::new("Exclude dirs from search")
            .paths(workdir.unwrap_or_default())
            .recurse("yes")
            .hidden("yes")
            .file_type("directory")
            .exclude(".git")
            .exclude("node_modules")
            .exclude(".vscode")
            .register("filtered_dirs")
            .tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn filter_dirs(tag: TaskTag) -> DefinedTask {
        FactTask::new(
            "dirs",
            "{{ filtered_dirs | json_query(\"files[*].path\") }}",
        )
        .tags(Some(vec![tag.to_string()]))
        .build()
    }

    fn gather_template_paths(tag: TaskTag) -> DefinedTask {
        FindTask::new("Gather all template paths")
            .paths("{{ dirs }}")
            .recurse("no")
            .hidden("true")
            .contains(DEFAULT_ANSIBLE_TEMPLATE_REGEX)
            .register("found_templates")
            .file_type("file")
            .tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn save_found_as_fact(tag: TaskTag) -> DefinedTask {
        FactTask::new(
            "template_paths",
            "{{ found_templates | json_query(\"files[*].path\") }}",
        )
        .tags(Some(vec![tag.to_string()]))
        .build()
    }

    fn write_template_paths_to_file(tag: TaskTag, manifest: Manifest) -> DefinedTask {
        let workdir = manifest.workdir;
        let paths_file = manifest.template_paths_file;
        let write_out = format!(
            "{}/{}",
            workdir.clone().unwrap_or_default(),
            paths_file.clone().unwrap_or_default()
        );
        CopyTask::new("Write template map")
            .content("{{ template_paths }}")
            .dest(write_out.as_str())
            .set_tags(Some(vec![tag.to_string()]))
            .build()
    }
}
