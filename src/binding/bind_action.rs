use std::ffi::{c_char, CStr};
use crate::action::action_type::{ActionBeginEnd, ActionType};
use crate::player::automata::automaton_action_str;
use num::FromPrimitive;



#[no_mangle]
pub extern "C" fn action(
    _name:*const c_char,
    _action_type:u64,
    _action_begin_end:u64,
    _source_node_id:u64,
    _dest_node_id:u64,
    _message_json_string:*const c_char,
) {
    let opt:Option<ActionType> = FromPrimitive::from_u64(_action_type);
    let action_type = opt.unwrap();
    let opt:Option<ActionBeginEnd> = FromPrimitive::from_u64(_action_begin_end);
    let action_begin_end = opt.unwrap();
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let json = unsafe { CStr::from_ptr(_message_json_string) }.to_str().unwrap().to_string();
    automaton_action_str(
        name.as_str(), action_type,
        action_begin_end,
        _source_node_id,
        _dest_node_id,
        json.as_str()
    )
}

