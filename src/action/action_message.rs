use bincode::Decode;
use bincode::Encode;
use scupt_util::message::{Message, MsgTrait};
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use serde::Deserialize;
use serde::Serialize;
use crate::action::action_serde_json_string::ActionSerdeJsonString;
use crate::action::action_type::ActionType;
use crate::action::res_serde::res_serde;

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
pub enum ActionMessage<
    Payload: MsgTrait + 'static,

> {
    // check states
    #[serde(bound = "Payload: MsgTrait")]
    Check(Message<Payload>),
    // initialize setup
    #[serde(bound = "Payload: MsgTrait")]
    Setup(Message<Payload>),
    // when check_all_begin_end disable, the driver would expected to send only end input action
    #[serde(bound = "Payload: MsgTrait")]
    Input(Message<Payload>),
    #[serde(bound = "Payload: MsgTrait")]
    // when check_all_begin_end disable, the driver would expected to send only begin input action
    Output(Message<Payload>),
    #[serde(bound = "Payload: MsgTrait")]
    Internal(Message<Payload>),
}

impl<
    Payload: MsgTrait + 'static

> MsgTrait for ActionMessage<Payload> {}

impl <Payload: MsgTrait + 'static>ActionMessage<Payload> {

    pub fn from_message(action_type:ActionType, message:Message<Payload>) -> Self {
        match action_type {
            ActionType::Check => { ActionMessage::Check(message) }
            ActionType::Setup => { ActionMessage::Setup(message) }
            ActionType::Input => { ActionMessage::Input(message) }
            ActionType::Internal => { ActionMessage::Internal(message) }
            ActionType::Output => { ActionMessage::Output(message)}
        }
    }

    pub fn from_payload(action_type: ActionType, source:NID, dest:NID, payload:Payload) -> Self {
        let message = Message::new(payload, source, dest);
        Self::from_message(action_type, message)
    }

    pub fn from_json_string(json_string:String) -> Res<Self> {
        let r = serde_json::from_str(json_string.as_str());
        let s = res_serde(r)?;
        Ok(s)
    }

    pub fn source_node_id(&self) -> Res<NID> {
        self.fn_message(|m|{ Ok(m.source()) })
    }

    pub fn dest_node_id(&self) -> Res<NID> {
        self.fn_message(|m|{ Ok(m.dest()) })
    }

    pub fn action_type(&self) -> ActionType {
        match self {
            ActionMessage::Check(_) => { ActionType::Check }
            ActionMessage::Setup(_) => { ActionType::Setup }
            ActionMessage::Input(_) => { ActionType::Input }
            ActionMessage::Output(_) => { ActionType::Output }
            ActionMessage::Internal(_) => { ActionType::Internal }
        }
    }

    fn fn_message<F, R>(&self, f:F) -> Res<R>
        where F: Fn(&Message<Payload>) -> Res<R> {
        match self {
            ActionMessage::Check(m) => { f(m) }
            ActionMessage::Setup(m) => { f(m) }
            ActionMessage::Input(m) => { f(m) }
            ActionMessage::Output(m) => { f(m) }
            ActionMessage::Internal(m) => { f(m)}
        }
    }

    pub fn to_serde_json_string(&self) -> Res<ActionSerdeJsonString> {
        let r = serde_json::to_string(self);
        let s = res_serde(r)?;
        Ok(ActionSerdeJsonString::new(s))
    }
}