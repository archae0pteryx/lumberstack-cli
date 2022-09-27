use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playbook {
  pub hosts: String,
  pub connection: String,
  pub tasks: Vec<Task>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatPath { pub path: String }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterTask {
  pub name: String,
  pub stat: StatPath,
  pub register: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitRemote {
  pub repo: String,
  pub dest: String,
  pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloneTask {
  pub name: String,
  pub git: GitRemote,
  pub when: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Task {
  Register(RegisterTask),
  GitClone(CloneTask)
}

// - hosts: localhost
//   connection: local

//   tasks:
//     - name: Register template dir
//       stat:
//         path: "/tmp/redwood-template-app"
//       register: template_dir

//     - name: Git checkout
//       git:
//         repo: 'https://github.com/codingzeal/redwood-template-app'
//         dest: /tmp/redwood-template-app
//         version: v0.0.1
//       when: not template_dir.stat.exists
