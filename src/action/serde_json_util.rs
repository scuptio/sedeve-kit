use scupt_util::error_type::ET;
use scupt_util::res::Res;
use serde_json::{Map, Value};
use tracing::error;

pub fn json_util_map_get_value(map: &Map<String, Value>, key: &str) -> Res<Value> {
    match map.get(key) {
        None => {
            error!("map get value, cannot find key {}, in map {:?}", key, map);
            Err(ET::NoneOption)
        }
        Some(v) => { Ok(v.clone()) }
    }
}

pub fn json_util_map_get_value_ref<'a>(map: &'a Map<String, Value>, key: &str) -> Res<&'a Value> {
    match map.get(key) {
        None => {
            error!("map get value reference, cannot find key {}", key);
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

#[cfg(test)]
mod test {
    use scupt_util::error_type::ET;
    use serde_json::json;
    use crate::action::serde_json_util::{json_util_map_get_i64, json_util_map_get_string, json_util_map_get_value, json_util_map_get_value_ref};

    #[test]
    fn test_serde_json_util() {
        let j = json!({
        "i64":1,
        "string":"s",
        "value" : {
            "x" : 1,
            "y" : 2
        }
    });

        let map = j.as_object().unwrap();

        // OK
        assert_eq!(Ok(1), json_util_map_get_i64(map, "i64"));
        assert_eq!(Ok("s".to_string()), json_util_map_get_string(map, "string"));
        let r1 = json_util_map_get_value(map, "value");
        assert!(r1.is_ok());
        let r1 = json_util_map_get_value_ref(map, "value");
        assert!(r1.is_ok());

        // non exist
        assert_eq!(Err(ET::NoneOption), json_util_map_get_i64(map, "non_exist"));
        assert_eq!(Err(ET::NoneOption), json_util_map_get_string(map, "non_exist"));
        assert_eq!(Err(ET::NoneOption), json_util_map_get_value(map, "non_exist"));
        assert_eq!(Err(ET::NoneOption), json_util_map_get_value(map, "non_exist"));

        // wrong types
        assert_eq!(Err(ET::NoneOption), json_util_map_get_i64(map, "string"));
        assert_eq!(Err(ET::NoneOption), json_util_map_get_string(map, "i64"));
    }
}