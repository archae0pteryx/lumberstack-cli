use serde::{Deserialize, Serialize};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DebugTask {
    pub debug: String,
}

impl DebugTask {
    pub fn create(registered_label: &str) -> DefinedTask {
        DefinedTask::Debug(DebugTask {
            debug: format!("var={}", registered_label),
        })
    }
}
