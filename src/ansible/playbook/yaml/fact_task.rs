use super::task_type::TaskType;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FactTask {
    set_fact: Map<String, Value>,
}

impl FactTask {
    pub fn new(key: &str, value: &str) -> FactTask {
        let mut map = Map::new();
        map.insert(key.to_string(), Value::String(value.to_string()));
        FactTask { set_fact: map }
    }

    #[allow(dead_code)]
    pub fn set(self: &Self, key: &str, value: &str) -> FactTask {
        let mut new_fact = self.set_fact.clone();
        new_fact.insert(key.to_string(), Value::String(value.to_string()));
        FactTask { set_fact: new_fact }
    }

    pub fn build(self: &Self) -> TaskType {
        TaskType::Fact(self.clone())
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
        assert!(matches!(built, TaskType::Fact {..}));
    }
}
