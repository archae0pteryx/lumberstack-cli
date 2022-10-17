use serde::{Deserialize, Serialize};

use crate::{
    commands::ShellCommand, spinner::create_spinner, lumberstack::Runnable, task_definitions::task_types::DefinedTask, app_config::DEFAULT_PLAYBOOK_FILE,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunnableAnsibleTask {
    #[serde(skip_serializing)]
    pub label: String,
    pub hosts: String,
    pub r#become: String,
    pub become_user: String,
    pub gather_facts: bool,
    pub tasks: Vec<DefinedTask>,
}

impl Runnable for RunnableAnsibleTask {
    fn run_job(&self) {
        let spinner = create_spinner(format!("{}", self.label));
        spinner.set_prefix("👟");
        self.write_yml();
        ShellCommand::exec("./", "ansible-playbook", &[&DEFAULT_PLAYBOOK_FILE], true);
        self.remove_playbook();
        spinner.finish_and_clear();
    }
}

impl RunnableAnsibleTask {
    pub fn new<T: AsRef<str>>(label: T) -> RunnableAnsibleTask {
        RunnableAnsibleTask {
            label: label.as_ref().to_string(),
            hosts: "localhost".to_string(),
            r#become: "yes".to_string(),
            become_user: r#"{{ lookup("env","USER") }}"#.to_string(),
            gather_facts: true,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn add_task(&self, task: DefinedTask) -> RunnableAnsibleTask {
        let mut new_playbook = self.clone();
        match task {
            DefinedTask::None() => {
                return new_playbook;
            }
            _ => {
                new_playbook.tasks.push(task);
                return new_playbook;
            }
        }
    }

    fn write_yml(&self) {
        let as_vec = vec![self.clone()];
        let yaml = serde_yaml::to_string(&as_vec).expect("Tried to create playbook as yaml");
        fs_extra::file::write_all(DEFAULT_PLAYBOOK_FILE, yaml.as_str())
            .expect("Tried to write playbook yaml to file. Could not");
    }

    fn remove_playbook(&self) {
        fs_extra::file::remove(DEFAULT_PLAYBOOK_FILE).expect("Tried to remove playbook. Could not");
    }
}
