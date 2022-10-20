use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    app_config::DEFAULT_PLAYBOOK_FILE, commands::ShellCommand, lumberstack::Runnable,
    spinner::create_spinner, task_definitions::task_types::DefinedTask,
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
        let spinner = create_spinner(&self.label);
        spinner.set_prefix("👟");

        self.write_yml();

        ShellCommand::exec("./", "ansible-playbook", &[DEFAULT_PLAYBOOK_FILE], true);
        Self::save_playbook(self);
    }
}

impl RunnableAnsibleTask {
    pub fn new<T: AsRef<str>>(label: T) -> Self {
        env::set_var("ANSIBLE_NOCOWS", "yes");
        env::set_var("ANSIBLE_ANY_ERRORS_FATAL", "yes");
        env::set_var("ANSIBLE_LOCALHOST_WARNING", "no");
        RunnableAnsibleTask {
            label: label.as_ref().to_string(),
            hosts: "localhost".to_string(),
            r#become: "yes".to_string(),
            become_user: r#"{{ lookup("env","USER") }}"#.to_string(),
            gather_facts: true,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: DefinedTask) -> &mut Self {
        match task {
            DefinedTask::None() => self,
            _ => {
                self.tasks.push(task);
                self
            }
        }
    }

    fn write_yml(&self) {
        let as_vec = vec![self.clone()];
        let yaml = serde_yaml::to_string(&as_vec).expect("Tried to create playbook as yaml");
        fs_extra::file::write_all(DEFAULT_PLAYBOOK_FILE, yaml.as_str())
            .expect("Tried to write playbook yaml to file. Could not");
    }

    fn save_playbook(&self) {
        let opts = fs_extra::file::CopyOptions::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let move_to = format!("tmp/{}_{}.yml", now.as_secs(), &self.label.replace(' ', ""));
        fs_extra::file::move_file(DEFAULT_PLAYBOOK_FILE, move_to, &opts).unwrap();
    }
}
