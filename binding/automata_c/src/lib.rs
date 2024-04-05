use std::ffi::{c_char, CStr, CString};
use sedeve_kit::action::action_type::{ActionBeginEnd, ActionType};
use sedeve_kit::dtm::automata;
use num::FromPrimitive;


/// Initialize an automata
/// `_name`, the automata's name
/// `_client_id`, client node id
/// `_server_id`, server(dtm player) node id
/// `_server_addr`, server(deterministic player to connect) network address [ip:port]
#[no_mangle]
pub extern "C" fn automata_init_setup(
    _name: *const c_char,
    _client_id:u64,
    _server_id: u64,
    _server_addr: *const c_char
) {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let server_addr = unsafe { CStr::from_ptr(_server_addr) }.to_str().unwrap().to_string();
    automata::automata_init_setup(
        name.as_str(),
        _client_id,
        _server_id,
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
/// `_output_action_type`, action type
/// `_output_buf_output_action_json`, output buffer
/// `_buf_len`, the buffer length
/// `_output_len`, write bytes to output buffer
#[no_mangle]
pub extern "C" fn automata_next_input(
    _name:*const c_char,
    _output_source_node_id:* mut u64,
    _output_dest_node_id:* mut u64,
    _output_action_type: *mut u64,
    _output_buf_output_action_json:*mut c_char,
    _buf_len:u64,
    _output_len:*mut u64
) -> i32 {
    let name = unsafe { CStr::from_ptr(_name) }.to_str().unwrap().to_string();
    let r = automata::automata_next_input(
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

/// Send an action to deterministic player
/// `_name`, the automata's name
/// `_action_type`, the action type, input/internal/output
/// `_action_begin_end`, begin an action, or end an action
/// `_source_node_id`, source node id
/// `_dest_node_id`, dest node id
/// `_message_json_string`, message in json reprensentaion
#[no_mangle]
pub extern "C" fn automata_action_to_player(
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
    automata::automata_send_action_to_player(
        name.as_str(), action_type,
        action_begin_end,
        _source_node_id,
        _dest_node_id,
        json.as_str()
    )
}

