use num::FromPrimitive;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use sedeve_kit::action::action_type::{ActionBeginEnd, ActionType};
use sedeve_kit::dtm::automata;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}


/// Initialize an automata
/// `_name`, the automata's name
/// `_client_id`, client node id
/// `_server_id`, server(dtm player) node id
/// `_server_addr`, server(deterministic player to connect) network address [ip:port]
#[pyfunction]
pub fn automata_init_setup(
    _name: String,
    _client_id: u64,
    _server_id: u64,
    _server_addr: String,
) {
    automata::automata_init_setup(
        _name.as_str(),
        _client_id,
        _server_id,
        _server_addr.as_str(),
    );
}

/// Clean an automata setting
/// `_name`, the automata's name
#[pyfunction]
pub fn automata_clear(_name: String) {
    automata::automata_clear(_name.as_str())
}

/// Is an automata named `name` enable
#[pyfunction]
pub fn automata_enable(_name: String) -> bool {
    automata::automata_enable(_name.as_str())
}

/// Read next input action of automata, receive message from deterministic player
/// `_name`, the automata's name
/// `_output_source_node_id`, source node id
/// `_output_dest_node_id`, dest node id
/// `_output_action_type`, action type
/// `_output_buf_output_action_json`, output buffer
/// `_buf_len`, the buffer length
/// `_output_len`, write bytes to output buffer
#[pyfunction]
pub fn automata_next_input(
    _name: String,
) -> PyResult<(u64, u64, u64, String)> {
    let r = automata::automata_next_input(
        _name.as_str()
    );
    return match r {
        Ok((source, dest, action_type, message)) => {
            Ok((source, dest, action_type as u64, message))
        }
        Err(_e) => {
            Err(PyTypeError::new_err(_e.to_string()))
        }
    };
}

/// Send an action to deterministic player
/// `_name`, the automata's name
/// `_action_type`, the action type, input/internal/output
/// `_action_begin_end`, begin an action, or end an action
/// `_source_node_id`, source node id
/// `_dest_node_id`, dest node id
/// `_message_json_string`, message in json reprensentaion
#[pyfunction]
pub fn automata_action_to_player(
    _name: String,
    _action_type: u64,
    _action_begin_end: u64,
    _source_node_id: u64,
    _dest_node_id: u64,
    _message_json_string: String,
) {
    let opt: Option<ActionType> = FromPrimitive::from_u64(_action_type);
    let action_type = opt.unwrap();
    let opt: Option<ActionBeginEnd> = FromPrimitive::from_u64(_action_begin_end);
    let action_begin_end = opt.unwrap();
    automata::automata_send_action_to_player(
        _name.as_str(),
        action_type,
        action_begin_end,
        _source_node_id,
        _dest_node_id,
        _message_json_string.as_str(),
    )
}

