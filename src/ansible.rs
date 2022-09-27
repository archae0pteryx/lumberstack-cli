use std::fs;

use clap::Parser;
use log::info;

use crate::{
    cli_args::CliArgs,
    commands::Commands,
    playbook::{CloneTask, GitRemote, Playbook, RegisterTask, StatPath, Task},
    System,
};

static TMP_DIR: &str = "/tmp";

static GITHUB_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
static GITHUB_REPO_NAME: &str = "redwood-template-app";
static GITHUB_PLAYBOOK_NAME: &str = "github-playbook.yml";
static GITHUB_TEMPLATE_VER: &str = "v0.0.1";

pub struct Ansible;
impl Ansible {
    pub fn run() {
        Self::create_github_playbook();
        Self::run_github_playbook();
    }

    fn run_github_playbook() {}

    fn create_github_playbook() {
        let mut tasks: Vec<Task> = Vec::new();
        let register_task = RegisterTask {
            name: "Register template dir".to_string(),
            stat: StatPath {
                path: format!("{}/{}", TMP_DIR, GITHUB_REPO_NAME),
            },
            register: "tmp_template_dir".to_string(),
        };
        let clone_task = CloneTask {
            name: "Clone template repo".to_string(),
            git: GitRemote {
                repo: GITHUB_TEMPLATE_REPO.to_string(),
                dest: format!("{}/{}", TMP_DIR, GITHUB_REPO_NAME),
                version: GITHUB_TEMPLATE_VER.to_string(),
            },
            when: "not tmp_template_dir.stat.exists".to_string(),
        };

        tasks.push(Task::Register(register_task));
        tasks.push(Task::GitClone(clone_task));

        let playbook = vec!(Playbook {
            hosts: "localhost".to_string(),
            connection: "local".to_string(),
            tasks,
        });
        let yaml = serde_yaml::to_string(&playbook).expect("Error converting github playbook yaml");
        let write_to = format!("{}/{}", TMP_DIR, GITHUB_PLAYBOOK_NAME);
        fs::write(write_to, yaml);
    }
}
