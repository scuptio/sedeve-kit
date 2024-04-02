use std::ffi::{c_char, CStr, CString};
use crate::action::action_type::{ActionBeginEnd, ActionType};
use crate::dtm::automata::{automaton_action_str, automaton_next_input_action};
use num::FromPrimitive;

#[no_mangle]
pub extern "C" fn automata_next_action(
    _name:*const c_char,
    _output_source_node_id:* mut u64,
    _output_dest_node_id:* mut u64,
    _output_action_type: *mut u64,
    _output_buf_output_action_json:*mut c_char,
    _buf_len:u64,
    _output_len:*mut u64
) -> i32 {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let r = automaton_next_input_action(
        name.as_str()
    );
    return match r {
        Ok((source, dest, action_type, message)) => {
            unsafe {*_output_source_node_id = source; };
            unsafe {*_output_dest_node_id = dest; };
            unsafe {*_output_action_type = action_type as u64 };
            let c_string = CString::new(message).unwrap();
            let len = c_string.to_bytes().len();
            if _buf_len as usize >= len {
                unsafe { _output_buf_output_action_json.copy_from(c_string.as_ptr(), len) }
            }

            0
        }
        Err(_e) => { -1 }
    }
}

#[no_mangle]
pub extern "C" fn automata_action(
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

