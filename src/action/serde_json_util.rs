use scupt_util::error_type::ET;
use scupt_util::res::Res;
use serde_json::{Map, Value};
use tracing::error;

pub fn json_util_map_get_value(map: &Map<String, Value>, key: &str) -> Res<Value> {
    match map.get(key) {
        None => {
            error!("map get string, cannot find key {}, in map {:?}", key, map);
            Err(ET::NoneOption)
        }
        Some(v) => { Ok(v.clone()) }
    }
}

pub fn json_util_map_get_value_ref<'a>(map: &'a Map<String, Value>, key: &str) -> Res<&'a Value> {
    match map.get(key) {
        None => {
            error!("map get string, cannot find key {}", key);
            Err(ET::NoneOption)
        }
        Some(v) => { Ok(v) }
    }
}

pub fn json_util_map_get_string(map: &Map<String, Value>, key: &str) -> Res<String> {
    let v = json_util_map_get_value(map, key)?;
    match v.as_str() {
        None => {
            error!("{:?} is not a string", v);
            return Err(ET::NoneOption);
        }
        Some(s) => {
            Ok(s.to_string())
        }
    }
}

pub fn json_util_map_get_i64(map: &Map<String, Value>, key: &str) -> Res<i64> {
    let v = json_util_map_get_value(map, key)?;
    match v.as_i64() {
        None => {
            error!("{:?} is not i64 type", v);
            return Err(ET::NoneOption);
        }
        Some(s) => {
            Ok(s)
        }
    }
}

