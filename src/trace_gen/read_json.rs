use scupt_util::res::Res;
use std::collections::HashMap;
use serde_json::Value;
use std::fs::read_to_string;
use scupt_util::error_type::ET;

pub fn read_from_dict_json(opt_path: Option<String>) -> Res<HashMap<String, Value>> {
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
