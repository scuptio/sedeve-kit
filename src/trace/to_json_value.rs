use serde_json::Value;

pub trait ToJsonValue {
    fn to_json_value(&self) -> Value;
}