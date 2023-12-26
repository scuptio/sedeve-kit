use std::collections::HashMap;

use scupt_util::mt_map::mt_map_from_vec;
use scupt_util::mt_set::mt_set_from_vec;
use scupt_util::res::Res;
use serde_json::{Map, Number, Value};
use tracing::debug;

use crate::action::constant::ACTION_LABEL;

#[derive(Debug, Clone)]
pub struct TLAVariableList {
    pub vec: Vec<TLAVarExpr>,
}

#[derive(Debug, Clone)]
pub enum TLANumber {
    TNFloat(f64),
    TNInteger(i64),
}

#[derive(Debug, Clone)]
pub enum TLAValue {
    TVRecord(Vec<(String, TLAValue)>),
    TVMap(Vec<(TLAValue, TLAValue)>),
    TVSet(Vec<TLAValue>),
    TVSequence(Vec<TLAValue>),
    TVNumberLiteral(TLANumber),
    TVStringLiteral(String),
    TVBoolean(bool),
}

#[derive(Debug, Clone)]
pub struct TLAVarExpr {
    pub name: String,
    pub value: TLAValue,
}


impl TLAVariableList {
    pub fn merge_list(&mut self, other: &mut TLAVariableList) {
        self.vec.append(&mut other.vec)
    }

    pub fn to_json(&self, const_word_dict: &HashMap<String, Value>) -> Option<Value> {
        for var in &self.vec {
            if var.name == ACTION_LABEL {
                let opt = tla_value_2_json(&var.value, const_word_dict);
                match opt {
                    Ok(v) => {
                        return Some(v);
                    }
                    Err(e) => {
                        panic!("to json error, {}", e.to_string())
                    }
                }
            }
        }
        return None;
    }
}

fn tla_value_set_2_json(set: &Vec<TLAValue>, const_word_dict: &HashMap<String, Value>) -> Res<Value> {
    let mut vec = vec![];
    for e in set {
        let v = tla_value_2_json(e, const_word_dict)?;
        vec.push(v);
    }
    let opt = mt_set_from_vec(vec);
    Ok(opt.unwrap())
}

fn tla_value_record_2_json(record: &Vec<(String, TLAValue)>, const_word_dict: &HashMap<String, Value>) -> Res<Value> {
    let mut map = Map::new();
    for (k, v) in record {
        let val = tla_value_2_json(v, const_word_dict)?;
        map.insert(k.clone(), val);
    }
    Ok(Value::Object(map))
}

fn tla_value_map_2_json(map: &Vec<(TLAValue, TLAValue)>, const_word_dict: &HashMap<String, Value>) -> Res<Value> {
    let mut vec = vec![];
    for (k, v) in map {
        let key = tla_value_2_json(k, const_word_dict)?;
        let val = tla_value_2_json(v, const_word_dict)?;
        vec.push((key, val));
    }
    debug!("map: {:?}", vec);
    let opt = mt_map_from_vec(vec);
    Ok(opt.unwrap())
}

fn tla_value_sequence_2_json(sequence: &Vec<TLAValue>, const_word_dict: &HashMap<String, Value>) -> Res<Value> {
    let mut vec = vec![];
    for e in sequence {
        let v = tla_value_2_json(e, const_word_dict)?;
        vec.push(v);
    }
    Ok(Value::Array(vec))
}

pub fn tla_value_2_json(action: &TLAValue, const_word_dict: &HashMap<String, Value>) -> Res<Value> {
    let value = match action {
        TLAValue::TVRecord(r) => { tla_value_record_2_json(r, const_word_dict)? }
        TLAValue::TVMap(m) => { tla_value_map_2_json(m, const_word_dict)? }
        TLAValue::TVSet(s) => { tla_value_set_2_json(s, const_word_dict)? }
        TLAValue::TVSequence(s) => { tla_value_sequence_2_json(s, const_word_dict)? }
        TLAValue::TVNumberLiteral(n) => { tla_value_number_literal_2_json(n)? }
        TLAValue::TVStringLiteral(s) => { tla_value_string_literal_2_json(s, const_word_dict)? }
        TLAValue::TVBoolean(b) => { tla_boolean_2_json(*b) }
    };
    Ok(value)
}

fn tla_value_number_literal_2_json(number: &TLANumber) -> Res<Value> {
    let v = match number {
        TLANumber::TNFloat(f) => { Value::Number(Number::from_f64(f.clone()).unwrap()) }
        TLANumber::TNInteger(i) => { Value::Number(Number::from(i.clone())) }
    };
    Ok(v)
}

fn tla_boolean_2_json(b: bool) -> Value {
    Value::Bool(b)
}

fn tla_value_string_literal_2_json(string: &String, const_word_dict: &HashMap<String, Value>) -> Res<Value> {
    match const_word_dict.get(string) {
        Some(v) => {
            Ok(v.clone())
        }
        None => {
            Ok(Value::String(string.clone()))
        }
    }
}
