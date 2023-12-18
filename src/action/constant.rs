use std::collections::HashMap;
use std::string::ToString;

use lazy_static::lazy_static;
use serde_json::{Number, Value};

pub const ACTION_LABEL: &str = "__action__";
pub const ACTION_FIELD_TYPE: &str = "t";
pub const ACTION_FIELD_NAME: &str = "name";

pub const ACTION_FIELD_PAYLOAD: &str = "p";
pub const MESSAGE_FIELD_PAYLOAD: &str = "payload";
pub const MESSAGE_FIELD_SOURCE: &str = "source";
pub const MESSAGE_FIELD_DEST: &str = "dest";
pub const LABEL: &str = "label";
pub const LOGICAL_CONJUNCTION: &str = "/\\";
pub const ACTION_SEQUENCE_FIELD_STATE_ID: &str = "i";
pub const ACTION_SEQUENCE_PREV_STATE_ID: &str = "p";

pub const ACTION_SEQUENCE_DEPENDENT_STATE_ID: &str = "d";

pub const ACTION_SEQUENCE_FIELD_ACTIONS: &str = "a";
pub const ACTION_SEQUENCE_FIELD_STATES: &str = "s";


/// Setup action type in .tla+
pub const ACTION_TYPE_SETUP: &str = "S";
/// Check action type in .tla+
pub const ACTION_TYPE_CHECK: &str = "C";
/// Input action type in .tla+
pub const ACTION_TYPE_INPUT: &str = "I";
/// Output action type in .tla+
pub const ACTION_TYPE_OUTPUT: &str = "O";
/// Internal action type in .tla+
pub const ACTION_TYPE_INTERNAL: &str = "T";


pub const SERDE_ACTION_TYPE_SETUP: &str = "Setup";
pub const SERDE_ACTION_TYPE_CHECK: &str = "Check";
pub const SERDE_ACTION_TYPE_INPUT: &str = "Input";
pub const SERDE_ACTION_TYPE_OUTPUT: &str = "Output";
pub const SERDE_ACTION_TYPE_INTERNAL: &str = "Internal";


pub const ACTION_NAME_SEPARATOR:&str = "::";


pub const __TEST_TLA_ACTION_VARIABLE_CONST: &str = r###"
/\ __action__ =
[
    i |-> 2,
    p |-> 1,
    s |-> <<>>,
    a |-> <<
        [
            p |->
                [
                    source |-> "A_n1",
                    dest |-> "A_n2",
                    name |-> "TestM1A::TestM2A",
                    payload |-> [x |-> 1, y |-> 2 , opt_z |-> "NULL_VALUE", ok |-> TRUE]
                ],
            t |-> "T"
        ],
        [
            p |->
                [
                    source |-> "A_n1",
                    dest |-> "A_n2",
                    name |-> "TestM1B::TestM2B",
                    payload |-> [x |-> 1, y |-> 2 , opt_z |-> 3, ok |-> FALSE]
                ],
            t |-> "I"
        ],
        [
            p |->
                [
                    source |-> "A_n1",
                    dest |-> "A_n2",
                    name |-> TestEmptyPayload
                ],
            t |-> "O"
        ]
        >>
]
"###;

lazy_static! {
pub static ref  __TEST_NODE_ID_MAP : HashMap<String, Value> = HashMap::from(
    [
        ("A_n1".to_string(), Value::Number(Number::from(1))),
        ("A_n2".to_string(), Value::Number(Number::from(2))),
        ("A_n3".to_string(), Value::Number(Number::from(3))),
        ("A_v1".to_string(), Value::Number(Number::from(1))),
        ("A_v2".to_string(), Value::Number(Number::from(2))),
        ("A_v3".to_string(), Value::Number(Number::from(3))),
        ("NULL_VALUE".to_string(), Value::Null),
        ("INVALID_NODE_ID".to_string(), Value::Null)
    ]
);
}