use serde::{Deserialize, Serialize};

use super::task_type::PlaybookYamlTaskType;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GitTask {
    pub name: String,
    pub git: GitRemote,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub when: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
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
            tags: None,
        }
    }

    pub fn when<S: AsRef<str>>(&self, when: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.when = when.as_ref().to_string();
        return new_task;
    }

    pub fn repo<S: AsRef<str>>(&self, repo: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.repo = repo.as_ref().to_string();
        return new_task;
    }

    pub fn dest<S: AsRef<str>>(&self, dest: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.dest = dest.as_ref().to_string();
        return new_task;
    }

    pub fn version<S: AsRef<str>>(&self, version: S) -> GitTask {
        let mut new_task = self.clone();
        new_task.git.version = version.as_ref().to_string();
        return new_task;
    }

    pub fn tags(&self, tags: Option<Vec<String>>) -> GitTask {
        let mut new_task = self.clone();
        new_task.tags = tags;
        return new_task;
    }

    pub fn build(&self) -> PlaybookYamlTaskType {
        PlaybookYamlTaskType::Git(self.clone())
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
        assert!(matches!(built, PlaybookYamlTaskType::Git { .. }));
    }
}
