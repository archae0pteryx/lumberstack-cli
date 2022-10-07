use serde::{Deserialize, Serialize};

use super::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GitRemote {
    pub repo: String,
    pub dest: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GitTask {
    pub name: String,
    pub git: GitRemote,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub when: String,
}

impl GitTask {
    pub fn new(name: &str) -> GitTask {
        GitTask {
            name: name.to_string(),
            when: String::new(),
            git: GitRemote::default(),
        }
    }

    pub fn when(self: &Self, when: &str) -> GitTask {
        let mut new_task = self.clone();
        new_task.when = when.to_string();
        return new_task;
    }

    pub fn repo(self: &Self, repo: &Option<String>) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.repo = repo.clone().unwrap_or_default();
        return new_task;
    }

    pub fn dest(self: &Self, dest: &Option<String>) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.dest = dest.clone().unwrap_or_default();
        return new_task;
    }

    pub fn version(self: &Self, version: &Option<String>) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.version = version.clone().unwrap_or_default();
        return new_task;
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::Git(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_git_task() {
        let actual = GitTask::new("foo").when("bar").repo(&Some("baz".to_string())).version(&Some("boing".to_string()));
        assert_eq!(actual.when, "bar");
        assert_eq!(actual.git.repo, "baz");
        assert_eq!(actual.git.version, "boing");

        let built = actual.build();
        assert!(matches!(built, TaskType::Git {..}));
    }
}
