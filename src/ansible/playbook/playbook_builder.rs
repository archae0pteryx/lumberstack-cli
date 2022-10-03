use serde::{Deserialize, Serialize};

use crate::{commands::Commands, DEFAULT_PLAYBOOK_FILE, DEFAULT_WORKDIR};

use super::yaml::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playbook {
    pub hosts: String,
    pub r#become: String,
    pub become_user: String,
    pub gather_facts: bool,
    pub tasks: Vec<TaskType>,
}

impl Playbook {
    pub(crate) fn new() -> Playbook {
        Playbook {
            hosts: "localhost".to_string(),
            r#become: "yes".to_string(),
            become_user: r#"{{ lookup("env","USER") }}"#.to_string(),
            gather_facts: true,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn add_task(self: &Self, task: TaskType) -> Playbook {
        let mut new_playbook = self.clone();
        match task {
            TaskType::None() => {
                return new_playbook;
            }
            _ => {
                new_playbook.tasks.push(task);
                return new_playbook;
            }
        }
    }

    pub(crate) fn run(self: &Self) {
        let as_vec = vec![self.clone()];
        let yaml = serde_yaml::to_string(&as_vec).expect("Tried to create playbook as yaml");
        fs_extra::file::write_all(DEFAULT_PLAYBOOK_FILE, yaml.as_str())
            .expect("Tried to write playbook yaml to file. Could not");
        Commands::exec_raw("./", "ansible-playbook", &[DEFAULT_PLAYBOOK_FILE], true);
        Self::remove_playbook();
    }

    fn remove_playbook() {
        fs_extra::file::remove(DEFAULT_PLAYBOOK_FILE).expect("Tried to remove playbook. Could not");
    }
}
