use std::collections::HashMap;

use scupt_util::error_type::ET;
use scupt_util::id::OID;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use serde_json::{Map, Number, Value};
use tracing::error;

use crate::action::{constant, serde_json_util};
use crate::action::action_serde_json_value::ActionSerdeJsonValue;
use crate::action::action_type::ActionType;
use crate::action::res_serde::res_serde;
use crate::action::tal_vars_parser::TLAVarsParser;
use crate::action::tla_typed_value::get_typed_enum;
use crate::action::tla_var_list_visitor::TLAVarListVisitor;

// the action sequence defined in TLA+
#[derive(
Clone,
Debug,
)]
pub struct TLAActionSeq {
    pub id_prev: i64,
    pub id: i64,
    pub states:Vec<TLAAction>,
    pub actions: Vec<TLAAction>,
}



#[derive(
Clone,
Debug,
)]
pub struct TLAAction {
    pub message:TLAMessage,
    pub action_type:ActionType,
}


// the message record defined in TLA+
#[derive(
Clone,
Debug,
)]
pub struct TLAMessage {
    pub source:OID,
    pub dest:OID,
    pub name:Vec<String>,
    pub payload:Value,
}

impl TLAActionSeq {
    pub fn field_prev_state_id(value: &Value) -> Res<i64> {
        let map = res_option(value.as_object())?;

        serde_json_util::json_util_map_get_i64(map, constant::ACTION_SEQUENCE_PREV_STATE_ID)
    }

    pub fn field_state_id(value: &Value) -> Res<i64> {
        let map = res_option(value.as_object())?;
        serde_json_util::json_util_map_get_i64(map, constant::ACTION_SEQUENCE_FIELD_STATE_ID)
    }

    fn field_actions(value: &Value) -> Res<&Value> {
        let map = res_option(value.as_object())?;
        serde_json_util::json_util_map_get_value_ref(map, constant::ACTION_SEQUENCE_FIELD_ACTIONS)
    }

    fn field_states(value: &Value) -> Res<&Value> {
        let map = res_option(value.as_object())?;
        serde_json_util::json_util_map_get_value_ref(map, constant::ACTION_SEQUENCE_FIELD_STATES)
    }

    pub fn from_str(string:&String) -> Res<Self> {
        let value:Value = res_serde(serde_json::from_str(string.as_str()))?;
        Self::from(value)
    }

    pub fn from(value:Value) -> Res<Self> {
        let id_prev = Self::field_prev_state_id(&value)?;
        let id = Self::field_state_id(&value)?;
        let value_actions = Self::field_actions(&value)?;
        let value_states = Self::field_states(&value)?;
        let mut state_action = vec![];
        for value in [value_states, value_actions] {
            let mut vec_tla_action = vec![];
            if value.is_array() {
                // a sequence of actions
                let opt = value.as_array();
                let array = res_option(opt)?;
                for v in array.iter() {
                    let action = TLAAction::from(v)?;
                    vec_tla_action.push(action);
                }
            } else if value.is_object() {
                let action = TLAAction::from(&value)?;
                vec_tla_action.push(action);
            }
            state_action.push(vec_tla_action);
        }

        let actions = state_action.pop().unwrap();
        let states = state_action.pop().unwrap();

        Ok(Self {
            id_prev,
            id,
            states,
            actions,
        })
    }

    pub fn actions(&self) ->  &Vec<TLAAction> {
        &self.actions
    }

    pub fn states(&self) -> &Vec<TLAAction> {
        &self.states
    }

    pub fn to_tuple(&self) -> Res<(i64, i64, String, String)> {
        let mut states = vec![];
        let mut actions = vec![];
        for state in &self.states {
            let v = state.to_action_json()?.action_json_value();
            states.push(v);
        }
        for action in &self.actions {
            let v = action.to_action_json()?.action_json_value();
            actions.push(v);
        }
        Ok((self.id, self.id_prev,
            serde_json::to_string(&states).unwrap(),
            serde_json::to_string(&actions).unwrap()))
    }
}

impl TLAAction {
    pub fn from(value :&Value) -> Res<Self> {
        let map = res_option(value.as_object())?;
        let s_action_type = serde_json_util::json_util_map_get_string(&map, constant::ACTION_FIELD_TYPE)?;
        let payload = serde_json_util::json_util_map_get_value(&map, constant::ACTION_FIELD_PAYLOAD)?;
        let message = TLAMessage::from(payload)?;
        Ok(TLAAction {
            message,
            action_type: ActionType::from_tla_action_type(s_action_type.as_str()),
        })
    }

    pub fn to_action_json(&self) -> Res<ActionSerdeJsonValue> {
        let payload = {
            let mut payload = self.message.payload.clone();
            payload = get_typed_enum(self.message.name.clone(), payload);
            payload
        };

        let message = {
            let mut map = Map::new();
            map.insert(constant::MESSAGE_FIELD_SOURCE.to_string(), Value::Number(Number::from(self.message.source)));
            map.insert(constant::MESSAGE_FIELD_DEST.to_string(), Value::Number(Number::from(self.message.dest)));
            map.insert(constant::MESSAGE_FIELD_PAYLOAD.to_string(), payload);
            Value::Object(map)
        };

        let action = {
            let mut map = Map::new();
            map.insert(self.action_type.to_string(), message);
            Value::Object(map)
        };
        ActionSerdeJsonValue::from_json_value(action)
    }
}

impl TLAMessage {
    pub fn from(value:Value) -> Res<Self> {
        if !value.is_object() {
            error!("Message payload JSON value must be an object,  JSON: {:?}", value);
            return Err(ET::ParseError("TLAMessage is not an object".to_string()))
        }
        let map = res_option(value.as_object())?;
        let source = serde_json_util::json_util_map_get_value(&map, constant::MESSAGE_FIELD_SOURCE)?;
        let dest = serde_json_util::json_util_map_get_value(&map, constant::MESSAGE_FIELD_DEST)?;
        let message_name = serde_json_util::json_util_map_get_string(&map, constant::ACTION_FIELD_NAME)?;
        let payload = if map.contains_key(constant::MESSAGE_FIELD_PAYLOAD) {
            serde_json_util::json_util_map_get_value(&map, constant::MESSAGE_FIELD_PAYLOAD)?
        } else {
            Value::Null
        };
        let vec: Vec<String> = message_name.split(constant::ACTION_NAME_SEPARATOR)
            .map(|s| { s.to_string() })
            .collect();
        Ok(Self {
            source:res_option(source.as_i64())? as OID,
            dest:res_option(dest.as_i64())? as OID,
            name: vec,
            payload,
        })
    }
}


pub fn tla_action_var_text_to_json(text:String, constant_dict_map:HashMap<String, Value>) -> Res<Value>{
    let mut parser = TLAVarsParser::new();
    let tree = parser.parse(&text)?;
    let visitor = TLAVarListVisitor::new(text.clone());
    let l = visitor.visit_root(tree.root_node())?;

    let v = res_option(l.to_json(&constant_dict_map))?;
    Ok(v)
}
