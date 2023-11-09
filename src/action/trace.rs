use scupt_util::error_type::ET;
use scupt_util::message::MsgTrait;
use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


use crate::action::action_message::ActionMessage;
use crate::action::res_serde::res_serde;

/// Trace definition
#[derive(
Clone,
Serialize,
Debug,
Deserialize,
)]
pub struct Trace<
    MsgPayload: MsgTrait + 'static
> {
    /// the first Action(must be an Input Action) possible be considered a init state
    /// member name must be consistent with const value F_EXECUTION_ACTIONS
    #[serde(bound = "MsgPayload: MsgTrait")]
    pub actions: Vec<ActionMessage<MsgPayload>>,
}



#[derive(
Clone,
Debug,
)]
pub struct TraceJsonValue {
    // actions is an array of json value
    pub actions: Vec<Value>,
}

impl<
    MsgPayload: MsgTrait + 'static
>
Trace<MsgPayload> {
    pub fn new() -> Self {
        Self {
            actions: vec![]
        }
    }

    pub fn to_json_string(&self) -> Res<String> {
        res_serde(serde_json::to_string(&self))
    }


    pub fn from_json_string(string:String) -> Res<Self> {
        res_serde(serde_json::from_str(string.as_str()))
    }
}

pub const F_EXECUTION_ACTIONS: &str = "actions";

impl TraceJsonValue {
    /// the json_string may be a json serde from
    ///    'trace::Trace<_> object' or 'trace::Trace<_>::action'
    pub fn from_json_string(s:String) -> Res<Self> {
        let value = res_serde(serde_json::from_str(s.as_str()))?;
        Self::from_json_value(value)
    }

    pub fn from_json_value(value : Value) -> Res<Self> {
        let value_array = match value {
            Value::Object(mut m) => {
                res_option(m.remove(F_EXECUTION_ACTIONS))?
            }
            Value::Array(v) => {
                Value::Array(v)
            }
            _ => { return Err(ET::SerdeError("error format".to_string())); }
        };
        let vec = match value_array {
            Value::Array(v) => {
                v
            }
            _ => {
                return Err(ET::SerdeError("error format".to_string()));
            }
        };
        Ok(Self {
            actions: vec,
        })
    }
}

