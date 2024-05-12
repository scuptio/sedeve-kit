use std::ffi::{c_char, CStr, CString};

use num::FromPrimitive;
use sedeve_kit::action::action_type::{ActionBeginEnd, ActionType};
use sedeve_kit::dtm::automata;

/// Set up an automata
/// `_name`, the automata's name
/// `_tested_nid`, tested node's node id
/// `_player_nid`, deterministic player's node id
/// `_player_addr`, deterministic player's network address [ip:port]
#[no_mangle]
pub extern "C" fn automata_setup(
    _name: *const c_char,
    _tested_nid:u64,
    _player_nid: u64,
    _player_addr: *const c_char
) {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let server_addr = unsafe { CStr::from_ptr(_player_addr) }.to_str().unwrap().to_string();
    automata::automata_setup(
        name.as_str(),
        _tested_nid,
        _player_nid,
        server_addr.as_str()
    );
}

/// Set up an automata
/// `_name`, the automata's name
/// `_tested_nid`, tested node's node id
/// `_tested_nid`, tested node's network address [ip:port]
/// `_player_nid`, deterministic player's node id
/// `_player_addr`, deterministic player's network address [ip:port]
#[no_mangle]
pub extern "C" fn automata_setup_with_input(
    _name: *const c_char,
    _tested_nid:u64,
    _tested_addr:*const c_char,
    _player_nid: u64,
    _player_addr: *const c_char
) {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let tested_addr = unsafe { CStr::from_ptr(_tested_addr) }.to_str().unwrap().to_string();
    let server_addr = unsafe { CStr::from_ptr(_player_addr) }.to_str().unwrap().to_string();
    automata::automata_setup_with_input(
        name.as_str(),
        _tested_nid,
        tested_addr.as_str(),
        _player_nid,
        server_addr.as_str()
    );
}

/// Clean an automata setting
/// `_name`, the automata's name
#[no_mangle]
pub extern "C" fn automata_clear(_name: *const c_char) {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    automata::automata_clear(name.as_str())
}


/// Close input channel
/// `_name`, the automata's name
#[no_mangle]
pub extern "C" fn automata_close_input(_name: *const c_char) {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    automata::automata_close_input(name.as_str())
}

/// Is an automata named `name` enable
#[no_mangle]
pub extern "C" fn automata_enable(_name: *const c_char) -> i32 {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let enable = automata::automata_enable(name.as_str());
    if enable {
        1
    } else {
        0
    }
}

/// Read next input action of automata, receive message from deterministic player
/// `_name`, the automata's name
/// `_output_source_node_id`, source node id
/// `_output_dest_node_id`, dest node id
/// `_output_buf_output_action_json`, output buffer
/// `_buf_len`, the buffer length
/// `_output_len`, write bytes to output buffer
#[no_mangle]
pub extern "C" fn automata_next_input(
    _name:*const c_char,
    _output_source_node_id:* mut u64,
    _output_dest_node_id:* mut u64,
    _output_buf_output_action_json:*mut c_char,
    _buf_len:u64,
    _output_len:*mut u64
) -> i32 {

    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let r = automata::automata_next_input(
        name.as_str()
    );

    return match r {
        Ok((source, dest, message)) => {
            unsafe {*_output_source_node_id = source; };
            unsafe {*_output_dest_node_id = dest; };
            let c_string = CString::new(message).unwrap();
            let len = c_string.to_bytes().len();
            if _buf_len as usize >= len {
                unsafe { _output_buf_output_action_json.copy_from(c_string.as_ptr(), len) }
            }
            unsafe { *_output_len = len as u64; };
            0
        }
        Err(_e) => { -1 }
    }
}

/// Send an action to deterministic player
/// `_name`, the automata's name
/// `_action_type`, the action type, input/internal/output
/// `_action_begin_end`, begin an action, or end an action
/// `_source_node_id`, source node id
/// `_dest_node_id`, dest node id
/// `_message_json_string`, message in json reprensentaion
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
    let action_type = match opt {
        Some(t) => { t }
        None => { panic!("unknown action type, {}", _action_type); }
    };

    let opt:Option<ActionBeginEnd> = FromPrimitive::from_u64(_action_begin_end);
    let action_begin_end = match opt {
        Some(t) => { t }
        None => { panic!("unknown begin or end id, {}", _action_begin_end); }
    };

    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let json = unsafe { CStr::from_ptr(_message_json_string) }.to_str().unwrap().to_string();
    automata::automata_send_action_to_player(
        name.as_str(), action_type,
        action_begin_end,
        _source_node_id,
        _dest_node_id,
        json.as_str()
    )
}

