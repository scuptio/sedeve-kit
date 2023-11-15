use std::collections::HashMap;
use rusqlite::Connection;
use scupt_util::res::Res;
use scupt_util::res_of::{res_parse, res_sqlite};
use serde_json::Value;
use tracing::info;
use crate::action::tla_actions::TLAActionSeq;
use crate::action::tla_typed_value::get_typed_value;
use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::action_node::ActionNode;
use crate::trace_gen::graph_util::adj_add_new_edge;
use crate::trace_gen::to_json_value::ToJsonValue;

pub fn read_from_state_db(path:String, dict: HashMap<String, Value>) -> Res<ActionGraph<String, ActionNode>> {
    let conn = res_sqlite(Connection::open(path))?;
    let mut stmt = res_sqlite(conn.prepare("select finger_print, json_string from state;"))?;
    let mut rows = res_sqlite(stmt.query([]))?;
    let mut adj = HashMap::new();
    let mut nodes = HashMap::new();
    while let Some(row) = res_sqlite(rows.next())? {
        let json:String = res_sqlite(row.get(1))?;
        let value : Value = res_parse(serde_json::from_str(json.as_str()))?;
        let value = get_typed_value(value, &dict)?;
        let id = TLAActionSeq::field_state_id(&value)?;
        let node = ActionNode::new(value);
        nodes.insert(id, node);
    }

    for (_k, v) in nodes.iter() {
        let prev_id = TLAActionSeq::field_prev_state_id(&v.to_json_value())?;
        let id = TLAActionSeq::field_state_id(&v.to_json_value())?;
        if prev_id.is_empty() {
            info!("{} -> {}", prev_id, id);
        }
        if nodes.contains_key(&prev_id) && nodes.contains_key(&id) {
            adj_add_new_edge(&mut adj, &prev_id, &id);
        }
    }
    Ok(ActionGraph::new(nodes, adj))
}