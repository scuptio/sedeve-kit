use num::FromPrimitive;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use sedeve_kit::action::action_type::{ActionBeginEnd, ActionType};
use sedeve_kit::action::action_type;
use sedeve_kit::dtm::automata;

#[pyfunction]
pub fn action_begin() -> PyResult<u64> {
    Ok(action_type::C_ACTION_BEGIN)
}

#[pyfunction]
pub fn action_end() -> PyResult<u64> {
    Ok(action_type::C_ACTION_END)
}

#[pyfunction]
pub fn action_input() -> PyResult<u64> {
    Ok(action_type::C_ACTION_INPUT)
}

#[pyfunction]
pub fn action_internal() -> PyResult<u64> {
    Ok(action_type::C_ACTION_INTERNAL)
}


#[pyfunction]
pub fn action_output() -> PyResult<u64> {
    Ok(action_type::C_ACTION_OUTPUT)
}

/// Initialize and set up an automata
/// `_name`, the automata's name
/// `_tested_nid`, tested node's node id
/// `_player_nid`, deterministic player's node id
/// `_player_addr`, deterministic player's network address [ip:port]
#[pyfunction]
pub fn automata_setup(
    _name: String,
    _tested_nid: u64,
    _player_nid: u64,
    _player_addr: String,
) {
    automata::automata_setup(
        _name.as_str(),
        _tested_nid,
        _player_nid,
        _player_addr.as_str(),
    );
}


/// Initialize and set up an automata
/// `_name`, the automata's name
/// `_tested_nid`, tested node's node id
/// `_player_addr`, tested node's network address [ip:port]
/// `_player_nid`, deterministic player's node id
/// `_player_addr`, deterministic player's network address [ip:port]
#[pyfunction]
pub fn automata_setup_with_input(
    _name: String,
    _tested_nid: u64,
    _tested_addr:String,
    _player_nid: u64,
    _player_addr: String,
) {
    automata::automata_setup_with_input(
        _name.as_str(),
        _tested_nid,
        _tested_addr.as_str(),
        _player_nid,
        _player_addr.as_str(),
    );
}

/// Clean an automata setting
/// `_name`, the automata's name
#[pyfunction]
pub fn automata_clear(_name: String) {
    automata::automata_clear(_name.as_str())
}

/// Close input channel
/// `_name`, the automata's name
#[pyfunction]
pub fn automata_close_input(_name: String) {
    automata::automata_close_input(_name.as_str())
}

/// Is an automata named `name` enable
#[pyfunction]
pub fn automata_enable(_name: String) -> bool {
    automata::automata_enable(_name.as_str())
}

/// Read next input action of automata, receive message from deterministic player
/// `_name`, the automata's name
/// `_source_node_id`, source node id
/// `_dest_node_id`, dest node id

/// `_output_json_message`, output message
#[pyfunction]
pub fn automata_next_input(
    _name: String,
) -> PyResult<(u64, u64, String)> {
    let r = automata::automata_next_input(
        _name.as_str()
    );
    return match r {
        Ok((source, dest, message)) => {
            Ok((source, dest, message))
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
/// `_message_json_string`, message in json representation
#[pyfunction]
pub fn automata_action(
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

/// A Python module implemented in Rust.
#[pymodule]
fn automata_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(action_begin, m)?)?;
    m.add_function(wrap_pyfunction!(action_end, m)?)?;
    m.add_function(wrap_pyfunction!(action_input, m)?)?;
    m.add_function(wrap_pyfunction!(action_internal, m)?)?;
    m.add_function(wrap_pyfunction!(action_output, m)?)?;

    m.add_function(wrap_pyfunction!(automata_setup, m)?)?;
    m.add_function(wrap_pyfunction!(automata_setup_with_input, m)?)?;
    m.add_function(wrap_pyfunction!(automata_clear, m)?)?;
    m.add_function(wrap_pyfunction!(automata_enable, m)?)?;
    m.add_function(wrap_pyfunction!(automata_next_input, m)?)?;
    m.add_function(wrap_pyfunction!(automata_action, m)?)?;
    Ok(())
}
