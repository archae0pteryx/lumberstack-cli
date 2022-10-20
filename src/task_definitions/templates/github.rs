use crate::{
    app_config::{AppConfig, DEFAULT_ANSIBLE_TEMPLATE_REGEX},
    system::logger::log_task_skip,
    task_definitions::{
        ansible::{
            ansible_task::RunnableAnsibleTask,
            yaml::{
                copy_task::CopyTask, fact_task::FactTask, find_task::FindTask, git_task::GitTask,
                register_task::RegisterTask,
            },
        },
        task_types::DefinedTask,
    },
};

use super::tags::{should_task_run, TaskTag};

#[derive(Clone)]

pub struct GithubTemplates;

impl GithubTemplates {
    pub fn create_runnable_task(
        tag: TaskTag,
        app_config: &AppConfig,
    ) -> Option<RunnableAnsibleTask> {
        if !should_task_run(&tag, app_config) {
            log_task_skip(tag.to_string());
            return None;
        }
        // Register a dir for ansible to manipulate
        let register_task = Self::register_template_dir(tag.clone(), app_config);
        // Clone the template repo
        let clone_task = Self::clone_template_repo(tag.clone(), app_config);
        // Exclude unnecessary dirs we search through and filter
        let exclude_dirs_task = Self::exclude_dirs_from_search(tag.clone(), app_config);
        // Filter the directories
        let filter_task = Self::filter_dirs(tag.clone());
        // After we filter we gather all the matching file paths for templates
        let gather_paths_task = Self::gather_template_paths(tag.clone());
        // Save the matching template paths as a var
        let save_fact_task = Self::save_found_as_fact(tag.clone());
        // Write the paths var to a file to be read by rust
        let write_paths_task = Self::write_template_paths_to_file(tag, app_config);
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
        .cloned()
    }

    fn register_template_dir(tag: TaskTag, app_config: &AppConfig) -> DefinedTask {
        RegisterTask::new("Register template dir")
            .register("tmp_templates")
            .stat_path(&app_config.template_dir)
            .tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn clone_template_repo(tag: TaskTag, app_config: &AppConfig) -> DefinedTask {
        let repo = &app_config.template_repo;
        let ver = &app_config.template_version;
        let template_path = &app_config.template_dir;

        GitTask::new("Clone template repo")
            .repo(repo)
            .dest(template_path)
            .version(ver)
            .when("not tmp_templates.stat.exists")
            .tags(Some(vec![tag.to_string()]))
            .build()
    }

    fn exclude_dirs_from_search(tag: TaskTag, app_config: &AppConfig) -> DefinedTask {
        let workdir = &app_config.workdir;
        FindTask::new("Exclude dirs from search")
            .paths(workdir)
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

    fn write_template_paths_to_file(tag: TaskTag, app_config: &AppConfig) -> DefinedTask {
        let paths_file = &app_config.template_map;
        CopyTask::new("Write template map")
            .content("{{ template_paths }}")
            .dest(paths_file.as_str())
            .set_tags(Some(vec![tag.to_string()]))
            .build()
    }
}