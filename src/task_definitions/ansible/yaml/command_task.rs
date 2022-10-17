#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandTask {
    pub name: String,
    pub command: CommandTaskCommand,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<CommandArgs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandTaskCommand {
    pub cmd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub creates: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandArgs {
    pub chdir: String,
}

impl CommandTask {
    pub fn new<S: AsRef<str>>(name: S) -> CommandTask {
        CommandTask {
            name: name.as_ref().to_string(),
            ..CommandTask::default()
        }
    }

    pub fn command<S: AsRef<str>>(&self, command: S) -> CommandTask {
        let mut new_task = self.clone();
        new_task.command.cmd = command.as_ref().to_string();
        return new_task;
    }

    pub fn chdir<S: AsRef<str>>(&self, chdir: S) -> CommandTask {
        let mut new_task = self.clone();
        let args = CommandArgs {
            chdir: chdir.as_ref().to_string(),
        };
        new_task.args = Some(args);
        return new_task;
    }

    pub fn register<S: AsRef<str>>(&self, register: S) -> CommandTask {
        let mut new_task = self.clone();
        new_task.register = Some(register.as_ref().to_string());
        return new_task;
    }

    pub fn creates<S: AsRef<str>>(&self, creates: S) -> CommandTask {
        let mut new_task = self.clone();
        new_task.command.creates = creates.as_ref().to_string();
        return new_task;
    }

    pub fn set_tags(&self, tags: Option<Vec<String>>) -> CommandTask {
        let mut new_task = self.clone();
        new_task.tags = tags;
        return new_task;
    }

    pub fn build(&self) -> DefinedTask {
        DefinedTask::Command(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_command_task() {
        let actual = CommandTask::new("foo").command("bar").chdir("baz");
        assert_eq!(actual.args.unwrap().chdir, "baz");
        assert_eq!(actual.name, "foo");
        assert_eq!(actual.command.cmd, "bar");
    }
}
