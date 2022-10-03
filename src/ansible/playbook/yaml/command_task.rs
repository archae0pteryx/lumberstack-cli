#![allow(unused)]
use serde::{Deserialize, Serialize};

use super::task_type::TaskType;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandTask {
    pub name: String,
    pub command: CommandTaskCommand,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<CommandArgs>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandTaskCommand {
    pub cmd: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub creates: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandArgs {
    pub chdir: String,
}

impl CommandTask {
    pub fn new(name: &str) -> CommandTask {
        CommandTask {
            name: name.to_string(),
            ..CommandTask::default()
        }
    }

    pub fn command(self: &Self, command: &str) -> CommandTask {
        let mut new_task = self.clone();
        new_task.command.cmd = command.to_string();
        return new_task;
    }

    pub fn chdir(self: &Self, chdir: &str) -> CommandTask {
        let mut new_task = self.clone();
        let args = CommandArgs {
            chdir: chdir.to_string(),
        };
        new_task.args = Some(args);
        return new_task;
    }

    pub fn register(self: &Self, register: &str) -> CommandTask {
        let mut new_task = self.clone();
        new_task.register = Some(register.to_string());
        return new_task;
    }

    pub fn creates(self: &Self, creates: &str) -> CommandTask {
        let mut new_task = self.clone();
        new_task.command.creates = creates.to_string();
        return new_task;
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::Command(self.clone())
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
