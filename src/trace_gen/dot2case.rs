use std::cell::RefCell;
use std::collections::HashMap;

use std::time::Instant;

use scupt_util::res::Res;
use serde_json::Value;
use tracing::{info, trace};

use crate::trace_gen::action_exec::ActionGraph;
use crate::trace_gen::parse_dot::parse_dot;
use crate::trace_gen::trace_db::TraceDB;
use crate::trace_gen::trace_builder::TraceBuilder;

pub fn dot_to_case(
    dot_path: String,
    db_path: String,
    remove_intermediate:bool,
    dict: HashMap<String, Value>
) -> Res<()> {

    info!("use const mapping: {:?}", dict);
    let inst = Instant::now();
    let action_graph = parse_dot(dot_path.clone(), dict)?;
    let duration = inst.elapsed();

    info!("Time elapsed to parse {}, time costs: {:?}", dot_path, duration);

    let inst = Instant::now();

    action_graph_output_to_db(&action_graph, db_path.clone())?;
    let duration = inst.elapsed();
    info!("Time elapsed to generate action trace, time costs: {:?}",  duration);

    let inst = Instant::now();
    TraceBuilder::build(db_path, remove_intermediate)?;

    let duration = inst.elapsed();
    info!("Time elapsed to gen final trace, time costs: {:?}", duration);
    Ok(())
}

fn action_graph_output_to_db(action_graph: &ActionGraph, db_path: String) -> Res<()> {
    let vec_path: RefCell<Vec<Vec<i64>>> = RefCell::new(vec![]);
    let vec_dump: RefCell<Vec<(i64, Value)>> = RefCell::new(vec![]);
    let mut trace_db = TraceDB::new(db_path)?;


    let f_p = |v: Vec<i64>| {
        trace!("find path :{:?}", v);
        let mut path = vec_path.borrow_mut();
        path.push(v);
    };
    let f_v = |v: i64, value: Value| {
        let mut dump = vec_dump.borrow_mut();
        dump.push((v, value));
    };

    let inst = Instant::now();
    action_graph.handle_action(&f_p, &f_v)?;
    let duration = inst.elapsed();
    info!("Time elapsed to handle action graph, time costs: {:?}",  duration);
    let mut db = trace_db.new_trans(true, false)?;
    {
        for path in vec_path.into_inner() {
            db.insert_path(path)?;
        }
        for (v, dump) in vec_dump.into_inner() {
            db.insert_action(v, dump)?;
        }
        db.commit()?;
    }

    Ok(())
}
