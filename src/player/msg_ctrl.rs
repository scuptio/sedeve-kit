use bincode::Decode;
use bincode::Encode;
use scupt_util::message::MsgTrait;
use scupt_util::res::Res;
use serde::Deserialize;
use serde::Serialize;

use crate::action::action_serde_json_string::ActionSerdeJsonString;

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
pub enum MessageControl {
    ActionReq {
        // a UUID let the DTM client to retrieve the response ActionACK message
        id:String,
        action: ActionSerdeJsonString,
        begin: bool
    },
    ActionACK {
        // a UUID
        id:String
    },
}


impl  MsgTrait for MessageControl {}

impl MessageControl {
    pub fn id(&self) -> Res<String> {
        match self {
            MessageControl::ActionReq { id, .. } => {
                Ok(id.clone())
            }
            MessageControl::ActionACK { id} => {
                Ok(id.clone())
            }
        }
    }
}