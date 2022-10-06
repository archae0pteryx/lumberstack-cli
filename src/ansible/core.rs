use crate::manifest::Manifest;
use indicatif::ProgressBar;

use super::{playbook::playbook_builder::Playbook, task_builders::AnsibleTasks};

pub struct Ansible;

impl Ansible {
    pub fn init_templates(manifest: Manifest, spinner: &ProgressBar) {
        let this_tag = "init";

        spinner.set_prefix("🚀");
        spinner.set_message("Lumberstack launching...");

        let should_run = Self::should_run(this_tag, &manifest);

        if should_run {
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

    pub fn create_redwood_app(manifest: Manifest, spinner: &ProgressBar) {
        let this_tag = "create";
        let should_run = Self::should_run(this_tag, &manifest);

        if should_run {
            spinner.set_prefix("🚀");
            spinner.set_message("Creating redwood app");

            Playbook::new()
                .add_task(AnsibleTasks::create_redwood_app(manifest))
                .run();
            spinner.set_message("Redwood created");
        }
    }

    pub fn generate_auth(manifest: Manifest, spinner: &ProgressBar) {
        let this_tag = "auth";
        let should_run = Self::should_run(this_tag, &manifest);

        if should_run {
            spinner.set_prefix("🔐");
            spinner.set_message("Generating auth");

            Playbook::new()
                .add_task(AnsibleTasks::setup_db_auth(manifest))
                .run();
            spinner.set_message("Auth generated");
        }
    }

    pub(crate) fn setup_docker(manifest: Manifest, spinner: &ProgressBar) {
        let this_tag = "db";
        let should_run = Self::should_run(this_tag, &manifest);

        if should_run {
            spinner.set_prefix("🐳");
            spinner.set_message("Setting up docker");
            Playbook::new()
                .add_task(AnsibleTasks::copy_compose(manifest))
                .run();
            spinner.set_message("Docker is setup");
        }
    }

    fn should_run(tag: &str, manifest: &Manifest) -> bool {
        let all_tags = &manifest.tags;
        let in_tags = Self::in_tags(tag, &manifest);
        let in_skip_tags = Self::in_skip_tags(tag, &manifest);

        if in_skip_tags {
            return false;
        }

        if in_tags || all_tags.is_none() {
            return true;
        }

        return false;
    }

    fn in_tags(tag: &str, manifest: &Manifest) -> bool {
        return manifest
            .clone()
            .tags
            .unwrap_or(Vec::new())
            .contains(&tag.to_string());
    }
    fn in_skip_tags(tag: &str, manifest: &Manifest) -> bool {
        return manifest
            .clone()
            .skip_tags
            .unwrap_or(Vec::new())
            .contains(&tag.to_string());
    }

    // pub(crate) fn collect_templates(manifest: Manifest, spinner: &&ProgressBar) {
    //     spinner.set_message("Collecting templates.");
    //     let paths_vec = Self::load_paths_vec(manifest.clone());
    //     for path in paths_vec {
    //         let mut loaded_file = fs_extra::file::read_to_string(&path).expect("Error loading template");
    //         // replace vars
    //         // if let Some(replace_map) = &manifest.replace {
    //         //     for (k, v) in replace_map {
    //         //         let format_var = format!("{{{}}}", k);
    //         //         loaded_file = loaded_file.replace(&format_var, v);
    //         //     }
    //         // }
    //         // src path
    //         let src_path = Path::new(&path);

    //         // dest path
    //         let dest_path = Self::build_template_destination(path, &manifest);

    //         fs_extra::file::write_all(dest_path, &loaded_file);
    //     }
    // }

    // fn load_paths_vec(manifest: Manifest) -> Vec<String> {
    //     let paths_file = manifest.template_paths_file.unwrap_or_default();
    //     let workdir = manifest.workdir.unwrap_or_default();
    //     let template_file_path = format!("{}/{}", workdir, paths_file);
    //     let paths_as_str = fs_extra::file::read_to_string(template_file_path).unwrap();
    //     let paths_vec: Vec<String> = serde_json::from_str(&paths_as_str).unwrap();
    //     return paths_vec;
    // }

    // pub(crate) fn build_template_destination(path: String, manifest: &Manifest) -> PathBuf {
    //     let app_name = manifest.clone().app_name.unwrap_or_default();
    //     let workdir = manifest.clone().workdir.unwrap_or_default();
    //     let template_dir = manifest.clone().template_dir.unwrap_or_default();
    //     let to_strip = format!("{}/{}", &workdir, &template_dir);
    //     let template_path = Path::new(&path);
    //     let project_path = template_path
    //         .strip_prefix(to_strip)
    //         .expect("Error stripping prefix from path");
    //     let app_path = Path::new(&app_name);
    //     let dest = app_path.join(project_path);
    //     return dest;
    // }
}
