use indicatif::ProgressBar;

use crate::{
    ansible::playbook::{
        playbook_builder::Playbook,
        yaml::{
            copy_task::CopyTask, fact_task::FactTask, find_task::FindTask, git_task::GitTask,
            register_task::RegisterTask, task_type::TaskType,
        },
    },
    manifest::Manifest,
    tags::should_task_run,
    TEMPLATE_TOKEN_REGEX,
};

static CLONE_TAG: &str = "clone";

pub struct CloneTemplates {
    this_tag: String,
    manifest: Manifest,
}

impl CloneTemplates {
    pub fn new(manifest: Manifest) -> (bool, Self) {
        let should_run = should_task_run(CLONE_TAG.to_string(), &manifest.tags);
        (
            should_run,
            CloneTemplates {
                this_tag: CLONE_TAG.to_string(),
                manifest,
            },
        )
    }

    pub fn build_playbook(self: &Self) -> Playbook {
        /// Register a dir for ansible to manipulate
        let register_task = self.register_template_dir();
        /// Clone the template repo
        let clone_task = self.clone_template_repo();
        /// Exclude unnecessary dirs we search through and filter
        let exclude_dirs_task = self.exclude_dirs_from_search();
        /// Filter the directories
        let filter_task = self.filter_dirs();
        /// After we filter we gather all the matching file paths for templates
        let gather_paths_task = self.gather_template_paths();
        /// Save the matching template paths as a var
        let save_fact_task = self.save_found_as_fact();
        /// Write the paths var to a file to be read by rust
        let write_paths_task = self.write_template_paths_to_file();
        /// The playbook combines them all together into a playbook that we can execute
        Playbook::new()
            .add_task(register_task)
            .add_task(clone_task)
            .add_task(exclude_dirs_task)
            .add_task(filter_task)
            .add_task(gather_paths_task)
            .add_task(save_fact_task)
            .add_task(write_paths_task)
    }

    fn register_template_dir(self: &Self) -> TaskType {
        let workdir = &self.manifest.workdir;
        let template_dir = &self.manifest.template_dir;
        let template_path = &self.manifest.full_template_path;
        RegisterTask::new("Register template dir")
            .register("tmp_templates")
            .stat_path(template_path)
            .build()
    }

    fn clone_template_repo(self: &Self) -> TaskType {
        let workdir = &self.manifest.workdir;
        let template_dir = &self.manifest.template_dir;
        let repo = &self.manifest.template_repo;
        let ver = &self.manifest.template_version;
        let template_path = &self.manifest.full_template_path;

        GitTask::new("Clone template repo")
            .repo(repo)
            .dest(template_path)
            .version(ver)
            .when("not tmp_templates.stat.exists")
            .build()
    }

    fn exclude_dirs_from_search(self: &Self) -> TaskType {
        let workdir = &self.manifest.workdir;
        FindTask::new("Exclude dirs from search")
            .paths(&workdir)
            .recurse(&Some("yes".to_string()))
            .hidden("yes")
            .file_type("directory")
            .exclude(".git")
            .exclude("node_modules")
            .exclude(".vscode")
            .register("filtered_dirs")
            .build()
    }

    fn filter_dirs(self: &Self) -> TaskType {
        FactTask::new(
            "dirs",
            "{{ filtered_dirs | json_query(\"files[*].path\") }}",
        )
        .build()
    }

    fn gather_template_paths(self: &Self) -> TaskType {
        FindTask::new("Gather all template paths")
            .paths(&Some("{{ dirs }}".to_string()))
            .recurse(&Some("no".to_string()))
            .file_type("file")
            .hidden("true")
            .contains(TEMPLATE_TOKEN_REGEX)
            .register("found_templates")
            .file_type("file")
            .build()
    }

    fn save_found_as_fact(self: &Self) -> TaskType {
        FactTask::new(
            "template_paths",
            "{{ found_templates | json_query(\"files[*].path\") }}",
        )
        .build()
    }

    fn write_template_paths_to_file(self: &Self) -> TaskType {
        let workdir = &self.manifest.workdir;
        let paths_file = &self.manifest.template_paths_file;
        let write_out = format!(
            "{}/{}",
            workdir.clone().unwrap_or_default(),
            paths_file.clone().unwrap_or_default()
        );
        CopyTask::new("Write template map")
            .content("{{ template_paths }}")
            .dest(write_out.as_str())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest;

    use super::*;

    #[test]
    fn clone_playbook_without_tags_should_flag_to_run() {
        let mut manifest = Manifest::default();
        let without_any_tags = CloneTemplates::new(manifest);
        assert!(without_any_tags.0);
    }

    #[test]
    fn clone_playbook_with_its_own_tag_should_flag_to_run() {
        let mut manifest= Manifest::default();
        let vec_with_tag = Some(vec![CLONE_TAG.to_string()]);
        manifest.tags = vec_with_tag;

        let with_own_tag = CloneTemplates::new(manifest);
        assert!(with_own_tag.0);
    }

    #[test]
    fn clone_playbook_with_other_tags_should_not_flag_to_run() {
        let mut manifest= Manifest::default();
        let vec_with_tag = Some(vec!["not-my-tag".to_string()]);
        manifest.tags = vec_with_tag;

        let with_own_tag = CloneTemplates::new(manifest);
        assert!(!with_own_tag.0);
    }
}
