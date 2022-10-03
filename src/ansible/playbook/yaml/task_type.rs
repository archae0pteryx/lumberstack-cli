use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    copy_task::CopyTask, fact_task::FactTask, file_task::FileTask, find_task::FindTask,
    git_task::GitTask, register_task::RegisterTask,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TaskType {
    Copy(CopyTask),
    Custom(Value),
    Fact(FactTask),
    File(FileTask),
    Find(FindTask),
    Git(GitTask),
    Register(RegisterTask),
    None(),
}
