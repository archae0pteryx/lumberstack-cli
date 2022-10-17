use serde::{Deserialize, Serialize};

use super::{ansible::yaml::{
    command_task::CommandTask, copy_task::CopyTask, fact_task::FactTask, file_task::FileTask,
    find_task::FindTask, git_task::GitTask, register_task::RegisterTask,
}, templates::copy::TemplateCopy};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DefinedTask {
    Copy(CopyTask),
    Command(CommandTask),
    Fact(FactTask),
    File(FileTask),
    Find(FindTask),
    Git(GitTask),
    Register(RegisterTask),
    RustCopy(TemplateCopy),
    None(),
}
