use bincode::{Decode, Encode};
use scupt_util::message::MsgTrait;
use scupt_util::res::Res;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::action::action_serde_json_value::ActionSerdeJsonValue;

// ActionSerdeJsonString are used by tested node and action player node to communicate
// there are three types of actions,
// input, internal, and output, the string value are the json representative of MsgTrait object
// when the player not or the tested node receive this message, the node would serde this string to
// object.
// the action player node would not aware of the static type of the MsgTrait.

#[derive(
Clone,
Serialize,
Hash,
PartialEq,
Eq,
Debug,
Deserialize,
Decode,
Encode,
)]
pub struct ActionSerdeJsonString {
    string: String,
}


impl ActionSerdeJsonString {
    pub fn from_json_value(value:&Value) -> Self {
        let s = value.to_string();
        Self::new(s)
    }
    pub fn from_json_string(json_string:String) -> Self {
        Self {
            string: json_string,
        }
    }

    pub fn new(json_string:String) -> Self {
        Self::from_json_string(json_string)
    }

    pub fn to_action_serde_json_value(&self) -> Res<ActionSerdeJsonValue> {
        let v = serde_json::from_str(&self.string).unwrap();

        ActionSerdeJsonValue::from_json_value( v)
    }

    pub fn to_string(&self) -> Res<String> {
        Ok(self.string.clone())
    }
}

impl MsgTrait for ActionSerdeJsonString {

}