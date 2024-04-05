use bincode::Decode;
use bincode::Encode;
use scupt_util::message::MsgTrait;
use scupt_util::res::Res;
use scupt_util::serde_json_string::SerdeJsonString;
use serde::Deserialize;
use serde::Serialize;

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
        id: String,
        action: SerdeJsonString,
        begin: bool,
    },
    ActionACK {
        // a UUID
        id: String
    },
}


impl MsgTrait for MessageControl {}

impl MessageControl {
    pub fn id(&self) -> Res<String> {
        match self {
            MessageControl::ActionReq { id, .. } => {
                Ok(id.clone())
            }
            MessageControl::ActionACK { id } => {
                Ok(id.clone())
            }
        }
    }
}