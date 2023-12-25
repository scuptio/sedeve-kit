use std::collections::HashMap;
use scupt_util::error_type::ET;
use scupt_util::mt_map::mt_map_from_value;
use scupt_util::mt_set::mt_set_from_value;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use serde_json::{Map, Value};

use crate::action::{constant, tla_value_kind};

pub fn get_typed_value(value:Value, constant_dict_map:&HashMap<String, Value>) -> Res<Value> {
    let mut value = value;
    let map = res_option(value.as_object_mut())?;
    let kind = res_option(map.remove("kind"))?;
    let object = res_option(map.remove("object"))?;
    let kind_t = res_option(kind.as_u64())? as u32;
    let value = match kind_t {
        tla_value_kind::BOOL_VALUE |
        tla_value_kind::INT_VALUE |
        tla_value_kind::REAL_VALUE=> {
            object
        }
        tla_value_kind::STRING_VALUE |
        tla_value_kind::MODEL_VALUE  => {
            let s = object.as_str().unwrap().to_string();
            match constant_dict_map.get(&s) {
                Some(v) => { v.clone() }
                None => { object }
            }
        }
        tla_value_kind::TUPLE_VALUE => {
            get_tuple_value(object, constant_dict_map)?
        }
        tla_value_kind::SET_CAP_VALUE |
        tla_value_kind::SET_CUP_VALUE |
        tla_value_kind::SET_ENUM_VALUE |
        tla_value_kind::SET_OF_FCNS_VALUE |
        tla_value_kind::SET_OF_RCDS_VALUE |
        tla_value_kind::SET_OF_TUPLES_VALUE => {
            get_set_value(object, constant_dict_map)?
        }
        tla_value_kind::RECORD_VALUE => {
            get_record_value(object, constant_dict_map)?
        }
        tla_value_kind::FCN_RCD_VALUE |
        tla_value_kind::FCN_LAMBDA_VALUE
        => {
            get_fcn_value(object, constant_dict_map)?
        }
        _ => {
            return Err(ET::NoneOption)
        }
    };
    Ok(value)
}

fn get_fcn_value(value:Value, constant_dict_map:&HashMap<String, Value>) -> Res<Value> {
    let map = match value {
        Value::Object(m) => {m}
        _ => { return Err(ET::NoneOption); }
    };

    let mut vec = vec![];
    for (_k, v) in map {
        let mut kv = match v {
            Value::Object(m) => {m}
            _ => { return Err(ET::NoneOption); }
        };
        let domain = res_option(kv.remove("domain"))?;
        let value = res_option(kv.remove("value"))?;
        let domain1 = get_typed_value(domain, constant_dict_map)?;
        let value1 = get_typed_value(value, constant_dict_map)?;
        vec.push((domain1, value1))
    }

    // when the domain has only 1 string value with upper case letter
    // this would be an enum value
    let use_enum = if vec.len() == 1 {
        let val = vec[0].0.clone();
        if let Value::String(s) = &val {
            let vec: Vec<char> = s.chars().collect();
            let opt_char = vec.first();
            match opt_char {
                Some(t) => { t.is_uppercase() }
                None => { false }
            }
        } else {
            false
        }
    } else {
        false
    };
    let value = if use_enum {
        let key = vec[0].0.to_string();
        let enum_vec: Vec<String> = key.split(constant::ACTION_NAME_SEPARATOR)
            .map(|s| { s.to_string() })
            .collect();
        get_typed_enum(enum_vec, vec[0].1.clone())

    } else {
        mt_map_from_value(vec)?
    };
    Ok(value)
}

pub fn get_typed_enum(vec:Vec<String>, value:Value) -> Value {
    let mut ret = value;
    let mut map = Map::new();
    for n in vec.iter().rev() {
        let is_null = match &ret {
            Value::Null => {
                true
            }
            _ => {
                false
            }
        };
        if is_null {
            ret = Value::String(n.clone());
        } else {
            map.insert(n.clone(), ret);
            ret = Value::Object(map);
            map = Map::new();
        }
    }
    ret
}

fn _get_array(value:Value, constant_dict_map:&HashMap<String, Value>) -> Res<Vec<Value>> {
    let array = match value {
        Value::Array(v) => { v }
        _ => { return Err(ET::NoneOption); }
    };
    let mut vec = vec![];
    for v in array {
        let v1 = get_typed_value(v, &constant_dict_map)?;
        vec.push(v1);
    }
    Ok(vec)
}
fn get_tuple_value(value:Value, constant_dict_map:&HashMap<String, Value>) -> Res<Value> {
    let array = _get_array(value, constant_dict_map)?;
    Ok(Value::Array(array))
}

fn get_set_value(value:Value, constant_dict_map:&HashMap<String, Value>) -> Res<Value> {
    let array = _get_array(value, constant_dict_map)?;
    mt_set_from_value(array)
}

fn get_record_value(value:Value, constant_dict_map:&HashMap<String, Value>) -> Res<Value> {
    let map = match value {
        Value::Object(m) => m,
        _ => { return Err(ET::NoneOption); }
    };
    let mut map1 = Map::new();
    for (k, v) in map {
        let v1 = get_typed_value(v, &constant_dict_map)?;
        map1.insert(k, v1);
    }
    Ok(Value::Object(map1))
}