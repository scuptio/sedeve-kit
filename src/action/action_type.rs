use bincode::{Decode, Encode};
use num_derive::FromPrimitive;
use scupt_util::message::{Message, MsgTrait};
use serde::{Deserialize, Serialize};

use crate::action::action_message::ActionMessage;
use crate::action::constant;

pub const C_ACTION_BEGIN: u64 = 0;
pub const C_ACTION_END: u64 = 1;

pub const C_ACTION_SETUP: u64 = 0;
pub const C_ACTION_CHECK: u64 = 1;
pub const C_ACTION_INPUT: u64 = 2;
pub const C_ACTION_OUTPUT: u64 = 3;
pub const C_ACTION_INTERNAL: u64 = 4;


/// Action type definition
#[derive(
FromPrimitive,
Copy,
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
pub enum ActionType {
    /// Set up and initializes the state of a node
    Setup = 0,

    /// Check the state correctness of a node, used for asserting invariants
    Check = 1,

    /// Represent a node receiving an input message, from a network endpoint or a terminal, for example
    Input = 2,

    /// Represent a node sending an output message, to a network endpoint or a terminal, for example
    Output = 3,

    /// Represent an internal event in a node
    Internal = 4,
}

#[derive(
FromPrimitive,
Copy,
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
pub enum ActionBeginEnd {
    Begin = 0,
    End = 1,
}

impl ActionType {
    pub fn from_tla_action_type(str: &str) -> Self {
        match str {
            constant::ACTION_TYPE_OUTPUT => { ActionType::Output }
            constant::ACTION_TYPE_INPUT => { ActionType::Input }
            constant::ACTION_TYPE_INTERNAL => { ActionType::Internal }
            constant::ACTION_TYPE_SETUP => { ActionType::Setup }
            constant::ACTION_TYPE_CHECK => { ActionType::Check }
            _ => { panic!("unknown TLA+ action type error") }
        }
    }

    pub fn from_serde_action_type(str: &str) -> Self {
        match str {
            constant::SERDE_ACTION_TYPE_OUTPUT => { ActionType::Output }
            constant::SERDE_ACTION_TYPE_INPUT => { ActionType::Input }
            constant::SERDE_ACTION_TYPE_INTERNAL => { ActionType::Internal }
            constant::SERDE_ACTION_TYPE_SETUP => { ActionType::Setup }
            constant::SERDE_ACTION_TYPE_CHECK => { ActionType::Check }
            _ => { panic!("unknown serde action type error") }
        }
    }

    pub fn action_message<P: MsgTrait + 'static>(&self, m: Message<P>) -> ActionMessage<P> {
        match self {
            ActionType::Check => { ActionMessage::Check(m) }
            ActionType::Setup => { ActionMessage::Setup(m) }
            ActionType::Input => { ActionMessage::Input(m) }
            ActionType::Internal => { ActionMessage::Internal(m) }
            ActionType::Output => { ActionMessage::Output(m) }
        }
    }

    pub fn to_string(&self) -> String {
        let s = match self {
            ActionType::Check => { constant::SERDE_ACTION_TYPE_CHECK.to_string() }
            ActionType::Setup => { constant::SERDE_ACTION_TYPE_SETUP.to_string() }
            ActionType::Input => { constant::SERDE_ACTION_TYPE_INPUT.to_string() }
            ActionType::Internal => { constant::SERDE_ACTION_TYPE_INTERNAL.to_string() }
            ActionType::Output => { constant::SERDE_ACTION_TYPE_OUTPUT.to_string() }
        };
        s
    }
}