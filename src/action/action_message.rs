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


/// Action message definition
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
    /// Check the state correctness of a node, used for asserting invariants
    #[serde(bound = "Payload: MsgTrait")]
    Check(Message<Payload>),

    /// Set up and initializes the state of a node
    #[serde(bound = "Payload: MsgTrait")]
    Setup(Message<Payload>),

    /// Represent a node receiving an input message, from a network endpoint or a terminal, for example.
    /// When check_all_begin_end disable, the driver would expected to send only end input action.
    #[serde(bound = "Payload: MsgTrait")]
    Input(Message<Payload>),

    /// Represent a node sending an output message, to a network endpoint or a terminal, for example.
    /// When check_all_begin_end disable, the driver would expected to send only begin output action.
    #[serde(bound = "Payload: MsgTrait")]
    Output(Message<Payload>),

    /// Represent an internal event in a node
    #[serde(bound = "Payload: MsgTrait")]
    Internal(Message<Payload>),
}

impl<
    Payload: MsgTrait + 'static

> MsgTrait for ActionMessage<Payload> {}

impl <Payload: MsgTrait + 'static>ActionMessage<Payload> {

    /// Build an ActionMessage through ActionType and Message struct.
    pub fn from_message(action_type:ActionType, message:Message<Payload>) -> Self {
        match action_type {
            ActionType::Check => { ActionMessage::Check(message) }
            ActionType::Setup => { ActionMessage::Setup(message) }
            ActionType::Input => { ActionMessage::Input(message) }
            ActionType::Internal => { ActionMessage::Internal(message) }
            ActionType::Output => { ActionMessage::Output(message)}
        }
    }

    /// Build an ActionMessage through ActionType, source destination node id,  and message payload.
    pub fn from_payload(action_type: ActionType, source:NID, dest:NID, payload:Payload) -> Self {
        let message = Message::new(payload, source, dest);
        Self::from_message(action_type, message)
    }

    /// Build an ActionMessage through serde_json representing string.
    pub fn from_json_string(json_string:String) -> Res<Self> {
        let r = serde_json::from_str(json_string.as_str());
        let s = res_serde(r)?;
        Ok(s)
    }

    /// Source node id
    pub fn source_node_id(&self) -> Res<NID> {
        self.fn_message(|m|{ Ok(m.source()) })
    }

    /// Destination node id
    pub fn dest_node_id(&self) -> Res<NID> {
        self.fn_message(|m|{ Ok(m.dest()) })
    }

    /// Action type
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

    /// Serde this action message to a JSON string
    pub fn to_serde_json_string(&self) -> Res<ActionSerdeJsonString> {
        let r = serde_json::to_string(self);
        let s = res_serde(r)?;
        Ok(ActionSerdeJsonString::new(s))
    }
}