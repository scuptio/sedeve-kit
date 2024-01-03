use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use scupt_util::res::Res;
use serde_json::Value;
use tracing::info;

use crate::trace_gen::action_from_parse_dot::dot_action_to_db;
use crate::trace_gen::action_from_state_db::graph_read_actions_from_state_db;
use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::graph_find_path::gen_new_vertex_id;
use crate::trace_gen::trace_builder::{OptBuild, TraceBuilder};
use crate::trace_gen::trace_db_interm::{Stage, TraceDBInterm};

const PATH_WRITE_BATCH: usize = 1000;

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
    data_output: String,
    dict: HashMap<String, Value>,
    opt_intermediate_path:Option<String>,
    initialize_setup:bool
) -> Res<()> {
    let intermediate = match opt_intermediate_path {
        Some(p) => { p }
        None => {
            format!("{}.intermediate.db", data_output)
        }
    };
    info!("use const mapping: {:?}", dict);
    let inst = Instant::now();
    match data_input {
        DataInput::DotFile(path) => {
            info!("Read dot file {}", path);

            let graph = dot_action_to_db(path.clone(), dict, intermediate.clone())?;
            let duration = inst.elapsed();

            info!("Time elapsed to parse {}, time costs: {:?}", path, duration);

            let inst = Instant::now();
            action_graph_output_to_db(&gen_new_vertex_id, &graph, intermediate.clone())?;
            let duration = inst.elapsed();
            info!("Time elapsed to generate action trace, time costs: {:?}",  duration);
        }
        DataInput::StateDB(path) => {
            let graph = graph_read_actions_from_state_db(path.clone(), dict, intermediate.clone())?;
            let duration = inst.elapsed();
            info!("Time elapsed to read from state DB {} and write actions, time costs: {:?}", path, duration);

            let inst = Instant::now();
            action_graph_output_to_db(&gen_new_vertex_id, &graph, intermediate.clone())?;
            let duration = inst.elapsed();
            info!("Time elapsed to generate path, time costs: {:?}",  duration);
        }
    }


    let inst = Instant::now();
    let opt = OptBuild {
        initialize_setup,
    };
    TraceBuilder::build(intermediate, data_output, opt)?;

    let duration = inst.elapsed();
    info!("Time elapsed to gen final trace, time costs: {:?}", duration);
    Ok(())
}


fn action_graph_output_to_db<
    NV: Fn(&HashSet<i64>) -> i64,
>(
    fn_new_vertex: &NV,
    action_graph: &ActionGraph<i64>,
    db_path: String,
) -> Res<()> {
    let db = RefCell::new(TraceDBInterm::new(db_path, None, None)?);
    let write_path = {
        let db_ref = db.borrow();
        let stage = db_ref.get_state()?;
        match stage {
            Stage::WriteAction | Stage::GeneratePath => {
                db_ref.begin_generate_path()?;
                true
            }
            _ => { false }
        }
    };
    if write_path {
        info!("To write path to DB");
        let vec: RefCell<Vec<Vec<i64>>> = RefCell::new(
            Vec::with_capacity(PATH_WRITE_BATCH));

        let write_batch_to_db = || {
            let mut vec_ref = vec.borrow_mut();
            let mut batch = Vec::with_capacity(PATH_WRITE_BATCH);
            let db_ref = db.borrow();
            std::mem::swap(&mut batch, &mut vec_ref);
            db_ref.write_path(batch).unwrap();
        };
        let f_write_path = |v: Vec<i64>| {
            let write_to_db = {
                let mut vec_ref = vec.borrow_mut();
                if !v.is_empty() {
                    vec_ref.push(v);
                }

                vec_ref.len() > PATH_WRITE_BATCH
            };
            if write_to_db {
                write_batch_to_db();
            }
        };

        let inst = Instant::now();
        action_graph.handle_action(fn_new_vertex, &f_write_path)?;
        {
            write_batch_to_db();
        }

        let duration = inst.elapsed();
        {
            let db_ref = db.borrow();
            db_ref.end_generate_path()?;
        }
        info!("Time elapsed to handle action graph, time costs: {:?}",  duration);
    }
    Ok(())
}
