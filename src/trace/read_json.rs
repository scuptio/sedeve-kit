use std::collections::HashMap;
use std::fs::read_to_string;

use scupt_util::error_type::ET;
use scupt_util::res::Res;
use serde_json::Value;


/// read a json file,
/// construct a mapping from TLA+ constant to programming language typed value
/// (Rust for example, by json Value)
pub fn tla_constant_mapping(opt_path: Option<String>) -> Res<HashMap<String, Value>> {
    let path = match opt_path {
        Some(p) => { p }
        None => { return Ok(HashMap::new()); }
    };

    let read_r = read_to_string(path);
    let json_string = match read_r {
        Ok(s) => { s }
        Err(e) => { return Err(ET::IOError(e.to_string())); }
    };

    let value_r = serde_json::from_str(json_string.as_str());
    let value: Value = match value_r {
        Ok(value) => { value }
        Err(e) => { return Err(ET::JSONError(format!("read dict json error, {}", e.to_string()))); }
    };

    let dict = match value {
        Value::Object(m) => {
            let mut dict = HashMap::new();
            for (k, v) in m {
                dict.insert(k, v);
            }
            dict
        }
        _ => { return Err(ET::JSONError("not a object".to_string())); }
    };
    Ok(dict)
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use serde_json::json;
    use uuid::Uuid;
    use crate::trace::read_json::tla_constant_mapping;

    #[test]
    fn test_constant_mapping() {
        assert_eq!(Ok(HashMap::new()), tla_constant_mapping(None));
        assert!(tla_constant_mapping(Some("/tmp/json.non_existing_file".to_string())).is_err());
        let invalid_json =r#"{ "x": }"#;
        let path1 = format!("/tmp/test_const_mapping_valid_json.{}.json", Uuid::new_v4().to_string());
        fs::write(path1.clone(), invalid_json).unwrap();
        assert!(tla_constant_mapping(Some(path1)).is_err());

        let array_json = json!([1, 2, 3]);
        let path2 = format!("/tmp/test_const_mapping_array_json.{}.json", Uuid::new_v4().to_string());
        fs::write(path2.clone(), array_json.to_string()).unwrap();
        assert!(tla_constant_mapping(Some(path2)).is_err());
    }
}