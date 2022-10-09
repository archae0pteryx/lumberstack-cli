pub mod create;
pub mod yaml;

use serde::{Deserialize, Serialize};

use crate::{
    commands::Commands, lumberstack::Runnable, spinner::create_spinner, DEFAULT_PLAYBOOK_FILE,
};

use self::yaml::task_type::PlaybookYamlTaskType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playbook {
    #[serde(skip_serializing)]
    pub label: String,
    pub hosts: String,
    pub r#become: String,
    pub become_user: String,
    pub gather_facts: bool,
    pub tasks: Vec<PlaybookYamlTaskType>,
}

impl Runnable for Playbook {
    fn run_job(&self) {
        let spinner = create_spinner(format!("Running {}", self.label));
        self.write_yml();
        Commands::exec_raw("./", "ansible-playbook", &[DEFAULT_PLAYBOOK_FILE], true);
        self.remove_playbook();
        spinner.finish_and_clear();
    }
}

impl Playbook {
    pub fn new<T: AsRef<str>>(label: T) -> Playbook {
        Playbook {
            label: label.as_ref().to_string(),
            hosts: "localhost".to_string(),
            r#become: "yes".to_string(),
            become_user: r#"{{ lookup("env","USER") }}"#.to_string(),
            gather_facts: true,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn add_task(&self, task: PlaybookYamlTaskType) -> Playbook {
        let mut new_playbook = self.clone();
        match task {
            PlaybookYamlTaskType::None() => {
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
