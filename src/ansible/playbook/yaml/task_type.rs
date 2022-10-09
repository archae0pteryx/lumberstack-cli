use serde::{Deserialize, Serialize};

use super::{
    copy_task::CopyTask, fact_task::FactTask, file_task::FileTask, find_task::FindTask,
    git_task::GitTask, register_task::RegisterTask, command_task::CommandTask,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PlaybookYamlTaskType {
    Copy(CopyTask),
    Command(CommandTask),
    Fact(FactTask),
    File(FileTask),
    Find(FindTask),
    Git(GitTask),
    Register(RegisterTask),
    None(),
}
