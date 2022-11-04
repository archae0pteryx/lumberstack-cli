use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::task_definitions::task_types::DefinedTask;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FactTask {
    set_fact: Map<String, Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
}

impl FactTask {
    pub fn new<S: AsRef<str>>(key: S, value: S) -> FactTask {
        let mut map = Map::new();
        map.insert(
            key.as_ref().to_string(),
            Value::String(value.as_ref().to_string()),
        );
        FactTask {
            set_fact: map,
            tags: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn set<S: AsRef<str>>(&self, key: S, value: S) -> FactTask {
        let mut new_fact = self.clone();
        new_fact.set_fact.insert(
            key.as_ref().to_string(),
            Value::String(value.as_ref().to_string()),
        );
        new_fact
    }

    pub fn tags(&self, tags: &[String]) -> FactTask {
        let new_task = self.clone();
        FactTask {
            set_fact: new_task.set_fact,
            tags: tags.to_vec(),
        }
    }

    pub fn build(&self) -> DefinedTask {
        DefinedTask::Fact(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_fact_task() {
        let actual = FactTask::new("foo", "bar");
        assert_eq!(actual.set_fact.get("foo").unwrap(), "bar");
        let built = actual.build();
        assert!(matches!(built, DefinedTask::Fact { .. }));
    }
}
