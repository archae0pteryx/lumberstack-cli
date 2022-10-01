use crate::manifest::Manifest;

use super::playbook::yaml::{
    fact_task::FactTask, find_task::FindTask, git_task::GitTask,
    register_task::RegisterTask, task_type::TaskType, copy_task::CopyTask,
};

pub struct AnsibleTasks;

impl AnsibleTasks {
    pub(crate) fn register_template_dir(manifest: &Manifest) -> TaskType {
        RegisterTask::new("Register template dir")
            .register("tmp_templates")
            .stat_path(&manifest.template_dir)
            .build()
    }

    pub(crate) fn clone_template_repo(manifest: &Manifest) -> TaskType {
        GitTask::new("Clone template repo")
            .repo(&manifest.template_repo)
            .dest(&manifest.template_dir)
            .version(&manifest.template_version)
            .when("not tmp_templates.stat.exists")
            .build()
    }

    pub(crate) fn exclude_dirs_from_search(manifest: &Manifest) -> TaskType {
        FindTask::new("Exclude dirs from search")
            .paths(&manifest.workdir)
            .recurse("no")
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
            .paths("{{ dirs }}")
            .recurse("yes")
            .file_type("file")
            .hidden("true")
            .contains(r#"(\/\*|#|\<!--) template!?.*"#)
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

    pub(crate) fn write_template_paths_to_file(manifest: &Manifest) -> TaskType {
        CopyTask::new("Write template map")
            .content("{{ template_paths  }}")
            .dest(format!("{}/template_map.txt", &manifest.workdir).as_str())
            .build()
    }
}
