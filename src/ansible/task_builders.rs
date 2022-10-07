#![allow(unused)]
use crate::{manifest::Manifest, TEMPLATE_TOKEN_REGEX};

use super::playbook::yaml::{
    command_task::CommandTask, copy_task::CopyTask, fact_task::FactTask, find_task::FindTask,
    git_task::GitTask, register_task::RegisterTask, task_type::TaskType,
};

pub struct AnsibleTasks;

impl AnsibleTasks {
    pub(crate) fn register_template_dir(manifest: Manifest) -> TaskType {
        let workdir = &manifest.workdir.unwrap_or_default();
        let template_dir = &manifest.template_dir.unwrap_or_default();
        let template_path = &manifest.full_template_path;
        RegisterTask::new("Register template dir")
            .register("tmp_templates")
            .stat_path(template_path)
            .build()
    }

    pub(crate) fn clone_template_repo(manifest: Manifest) -> TaskType {
        let workdir = &manifest.workdir.unwrap_or_default();
        let template_dir = &manifest.template_dir.unwrap_or_default();
        let template_path = &manifest.full_template_path;
        let repo = &manifest.template_repo;
        let ver = &manifest.template_version;

        GitTask::new("Clone template repo")
            .repo(repo)
            .dest(template_path)
            .version(ver)
            .when("not tmp_templates.stat.exists")
            .build()
    }

    pub(crate) fn exclude_dirs_from_search(manifest: Manifest) -> TaskType {
        let workdir = &manifest.workdir;
        FindTask::new("Exclude dirs from search")
            .paths(workdir)
            .recurse("yes")
            .hidden("yes")
            .file_type("directory")
            .exclude(".git")
            .exclude("node_modules")
            .exclude(".vscode")
            .register("filtered_dirs")
            .build()
    }

    pub(crate) fn filter_dirs() -> TaskType {
        FactTask::new(
            "dirs",
            "{{ filtered_dirs | json_query(\"files[*].path\") }}",
        )
        .build()
    }

    pub(crate) fn gather_template_paths() -> TaskType {
        FindTask::new("Gather all template paths")
            .paths(&Some("{{ dirs }}".to_string()))
            .recurse("no")
            .file_type("file")
            .hidden("true")
            .contains(TEMPLATE_TOKEN_REGEX)
            .register("found_templates")
            .file_type("file")
            .build()
    }

    pub(crate) fn save_found_as_fact() -> TaskType {
        FactTask::new(
            "template_paths",
            "{{ found_templates | json_query(\"files[*].path\") }}",
        )
        .build()
    }

    pub(crate) fn write_template_paths_to_file(manifest: Manifest) -> TaskType {
        let workdir = &manifest.workdir.unwrap_or_default();
        let paths_file = &manifest.template_paths_file.unwrap_or_default();
        let write_out = format!("{}/{}", workdir, paths_file);
        CopyTask::new("Write template map")
            .content("{{ template_paths }}")
            .dest(write_out.as_str())
            .build()
    }

    pub(crate) fn create_redwood_app(manifest: Manifest) -> TaskType {
        let app_name = &manifest.app_name.unwrap_or_default();
        let workdir = &manifest.workdir.unwrap_or_default();
        let command = format!(
                "yarn create redwood-app {} --typescript --overwrite > {}/create.stdout.log 2> {}/create.stderr.log",
                app_name,
                workdir,
                workdir
            );
        CommandTask::new("Create redwood app")
            .command(command.as_str())
            .creates(app_name.as_str())
            .register("create_command")
            .build()
    }

    pub(crate) fn setup_db_auth(manifest: Manifest) -> TaskType {
        let app_name = manifest.app_name.unwrap_or_default();
        CommandTask::new("Setup redwood auth")
            .command("yarn rw setup auth dbAuth --force")
            .chdir(app_name.as_str())
            .build()
    }

    pub(crate) fn generate_auth(manifest: Manifest) -> TaskType {
        let app_name = manifest.app_name.unwrap_or_default();
        CommandTask::new("Generating auth")
            .command("echo fooabar")
            .chdir(app_name.as_str())
            .build()
    }

    pub(crate) fn copy_compose(manifest: Manifest) -> TaskType {
        let app_name = manifest.app_name.unwrap_or_default();
        CommandTask::new("").build()
    }
}
