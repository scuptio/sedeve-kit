use serde_json::Value;
use crate::trace_gen::to_json_value::ToJsonValue;

pub struct ActionNode {
    json_value: Value,
}

impl ActionNode {
    pub fn new(value: Value) -> Self {
        Self {
            json_value: value,
        }
    }
}

impl ToJsonValue for ActionNode {
    fn to_json_value(&self) -> Value {
        self.json_value.clone()
    }
}