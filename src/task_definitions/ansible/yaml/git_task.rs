use serde::{Deserialize, Serialize};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct GitTask {
    pub name: String,
    pub git: GitRemote,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub when: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct GitRemote {
    pub repo: String,
    pub dest: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub version: String,
}

impl GitTask {
    pub fn new(name: &str) -> GitTask {
        GitTask {
            name: name.to_string(),
            when: String::new(),
            git: GitRemote::default(),
            tags: vec![],
        }
    }

    pub fn when<S: AsRef<str>>(&self, when: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.when = when.as_ref().to_string();
        new_task
    }

    pub fn repo<S: AsRef<str>>(&self, repo: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.repo = repo.as_ref().to_string();
        new_task
    }

    pub fn dest<S: AsRef<str>>(&self, dest: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.dest = dest.as_ref().to_string();
        new_task
    }

    pub fn version<S: AsRef<str>>(&self, version: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.version = version.as_ref().to_string();
        new_task
    }

    pub fn tags(&self, tags: &[String]) -> GitTask {
        let mut new_task = self.clone();
        new_task.tags = tags.to_vec();
        new_task
    }

    pub fn build(&self) -> DefinedTask {
        DefinedTask::Git(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_git_task() {
        let actual = GitTask::new("foo").when("bar").repo("baz").version("boing");
        assert_eq!(actual.when, "bar");
        assert_eq!(actual.git.repo, "baz");
        assert_eq!(actual.git.version, "boing");

        let built = actual.build();
        assert!(matches!(built, DefinedTask::Git { .. }));
    }
}
