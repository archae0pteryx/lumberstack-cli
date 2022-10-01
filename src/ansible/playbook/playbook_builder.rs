use serde::{Deserialize, Serialize};

use crate::{commands::Commands, DEFAULT_WORKDIR};

use super::yaml::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playbook {
    pub hosts: String,
    pub connection: String,
    pub gather_facts: bool,
    pub tasks: Vec<TaskType>,
}

impl Playbook {
    pub(crate) fn new() -> Playbook {
        Playbook {
            hosts: "127.0.0.1".to_string(),
            connection: "local".to_string(),
            gather_facts: true,
            tasks: Vec::new(),
        }
    }

    pub(crate) fn add_task(self: &Self, task: TaskType) -> Playbook {
        let mut new_playbook = self.clone();
        new_playbook.tasks.push(task);
        return new_playbook;
    }

    pub(crate) fn run(self: &Self) {
        let as_vec = vec![self.clone()];
        let yaml = serde_yaml::to_string(&as_vec).expect("Error converting cor playbook yaml");
        let path = format!("{}/playbook.yml", DEFAULT_WORKDIR);
        fs_extra::file::write_all(&path, yaml.as_str()).expect("Cannot write playbook yaml");
        Commands::exec_raw("./", "ansible-playbook", &[path.as_str()], true)
    }
}
