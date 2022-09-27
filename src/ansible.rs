use clap::Parser;
use ignore::{DirEntry, Walk, WalkBuilder};
use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use std::io::BufRead;
use std::{
    collections::HashSet,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use crate::{
    cli_args::CliArgs,
    commands::Commands,
    playbook::{CloneTask, GitRemote, Playbook, RegisterTask, StatPath, Task},
    System,
};

static TMP_DIR: &str = "/tmp";

static GITHUB_TEMPLATE_REPO: &str = "https://github.com/codingzeal/redwood-template-app";
static GITHUB_CLONE_NAME: &str = "redwood-template-app";
static GITHUB_PLAYBOOK_NAME: &str = "github-playbook.yml";
static GITHUB_TEMPLATE_VER: &str = "v0.0.1";

pub struct Ansible;
impl Ansible {
    pub fn run() {
        let playbook_path = format!("{}/{}", TMP_DIR, GITHUB_PLAYBOOK_NAME);
        let repo_dir = format!("{}/{}", TMP_DIR, GITHUB_CLONE_NAME);

        Self::create_github_playbook(&playbook_path, &repo_dir);
        Self::run_github_playbook(&playbook_path);

        let files = Self::gather_all_files(&repo_dir);
        let templates = Self::extract_templates(files);
        println!("{:?}", templates);

        // Self::cleanup_github_playbook(&playbook_path, &repo_dir);
    }

    fn run_github_playbook(playbook: &String) {
        Commands::exec_raw("./", "ansible-playbook", &[playbook], true)
    }

    fn create_github_playbook(playbook_path: &String, repo_dir: &String) {
        let mut tasks: Vec<Task> = Vec::new();
        let register_task = RegisterTask {
            name: "Register template dir".to_string(),
            stat: StatPath {
                path: repo_dir.to_owned(),
            },
            register: "tmp_template_dir".to_string(),
        };
        let clone_task = CloneTask {
            name: "Clone template repo".to_string(),
            git: GitRemote {
                repo: GITHUB_TEMPLATE_REPO.to_string(),
                dest: repo_dir.to_owned(),
                version: GITHUB_TEMPLATE_VER.to_string(),
            },
            when: "not tmp_template_dir.stat.exists".to_string(),
        };

        tasks.push(Task::Register(register_task));
        tasks.push(Task::GitClone(clone_task));

        let playbook = vec![Playbook {
            hosts: "localhost".to_string(),
            connection: "local".to_string(),
            tasks,
        }];
        let yaml = serde_yaml::to_string(&playbook).expect("Error converting github playbook yaml");
        fs::write(playbook_path, yaml);
    }

    fn extract_templates(files: Vec<DirEntry>) -> Vec<DirEntry> {
        let template_paths: Vec<DirEntry> = files
            .into_iter()
            .filter(|f| {
                let file = fs::read_to_string(f.path());
                if let Ok(contents) = file {
                    return is_template(&contents);
                }
                return false;
            })
            .collect();
        return template_paths;
    }

    fn gather_all_files(repo_dir: &String) -> Vec<DirEntry> {
        let files: Vec<DirEntry> = WalkBuilder::new(repo_dir)
            .standard_filters(false)
            .add_custom_ignore_filename(".templateignore")
            .build()
            .filter_map(|f| f.ok())
            .collect();
        return files;
    }

    fn cleanup_github_playbook(playbook: &String, repo_dir: &String) {
        fs::remove_file(playbook);
        fs::remove_dir_all(repo_dir);
    }
}

fn is_template(file_str: &String) -> bool {
    // lazy static to not recompile regex over and over
    lazy_static! {
      /*  */
        static ref TEMPLATE_RE: Regex = Regex::new(r"/* template.* */").unwrap();
    }
    if let Some(_) = TEMPLATE_RE.find(file_str) {
        return true;
    }
    return false;
}
