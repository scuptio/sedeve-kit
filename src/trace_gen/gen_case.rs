use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use std::time::Instant;

use scupt_util::res::Res;
use serde_json::Value;
use tracing::{info, trace};
use uuid::Uuid;

use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::graph_find_path::gen_new_vertex_id;
use crate::trace_gen::action_from_parse_dot::parse_dot;
use crate::trace_gen::action_from_state_db::graph_read_actions_from_state_db;
use crate::trace_gen::to_json_value::ToJsonValue;
use crate::trace_gen::trace_db::TraceDB;
use crate::trace_gen::trace_builder::TraceBuilder;



pub enum DataInput {
    DotFile(String),
    StateDB(String),
}

impl DataInput {
    pub fn path(&self) -> String {
        match self {
            DataInput::DotFile(p) => { p.clone() }
            DataInput::StateDB(p) => { p.clone() }
        }
    }
}
pub fn gen_case(
    data_input: DataInput,
    db_path: String,
    remove_intermediate:bool,
    dict: HashMap<String, Value>
) -> Res<()> {
    info!("use const mapping: {:?}", dict);
    let inst = Instant::now();
    match data_input {
        DataInput::DotFile(path) => {
            info!("Read dot file {}", path);

            let action_graph = parse_dot(path.clone(), dict)?;
            let duration = inst.elapsed();

            info!("Time elapsed to parse {}, time costs: {:?}", path, duration);

            let inst = Instant::now();

            action_graph_output_to_db(&gen_new_vertex_id, &action_graph, db_path.clone())?;
            let duration = inst.elapsed();
            info!("Time elapsed to generate action trace, time costs: {:?}",  duration);
        }
        DataInput::StateDB(path) => {
            let action_graph = graph_read_actions_from_state_db(path.clone(), dict)?;
            let duration = inst.elapsed();

            info!("Time elapsed to read from state DB {}, time costs: {:?}", path, duration);

            let inst = Instant::now();
            let f_new_vertex = |set:&HashSet<String>| {
                loop {
                    let uuid = Uuid::new_v4().to_string();
                    if !set.contains(&uuid) {
                        return uuid;
                    }
                }
            };
            action_graph_output_to_db(&f_new_vertex, &action_graph, db_path.clone())?;
            let duration = inst.elapsed();
            info!("Time elapsed to generate action trace, time costs: {:?}",  duration);
        }
    }


    let inst = Instant::now();
    TraceBuilder::build(db_path, remove_intermediate)?;

    let duration = inst.elapsed();
    info!("Time elapsed to gen final trace, time costs: {:?}", duration);
    Ok(())
}

fn action_graph_output_to_db<
    K:Eq + Hash + Clone + Ord + Debug + ToString,
    V:ToJsonValue + 'static,
    // create a new vertex ID not in set
    NV: Fn(&HashSet<K>) -> K,
>(
    fn_new_vertex: &NV,
    action_graph:&ActionGraph<K, V>,
    db_path: String
) -> Res<()> {
    let vec_path: RefCell<Vec<Vec<K>>> = RefCell::new(vec![]);
    let vec_dump: RefCell<Vec<(K, Value)>> = RefCell::new(vec![]);
    let mut trace_db = TraceDB::new(db_path)?;


    let f_p = |v: Vec<K>| {
        trace!("find path :{:?}", v);
        let mut path = vec_path.borrow_mut();
        path.push(v);
    };
    let f_v = |v: K, value: Value| {
        let mut dump = vec_dump.borrow_mut();
        dump.push((v, value));
    };

    let inst = Instant::now();


    action_graph.handle_action(fn_new_vertex, &f_p, &f_v)?;
    let duration = inst.elapsed();
    info!("Time elapsed to handle action graph, time costs: {:?}",  duration);
    let mut db = trace_db.new_trans(true, false)?;
    {
        for path in vec_path.into_inner() {
            let p = path.iter().map(|e| e.to_string()).collect();
            db.insert_path(p)?;
        }
        for (v, dump) in vec_dump.into_inner() {
            db.insert_action(v.to_string(), dump)?;
        }
        db.commit()?;
    }

    Ok(())
}
