use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;

use scupt_util::res::Res;
use serde_json::Value;
use tracing::info;

use crate::trace::action_from_state_db::state_db_to_from_action;
use crate::trace::action_graph::ActionGraph;
use crate::trace::trace_builder::{OptBuild, TraceBuilder};
use crate::trace::trace_db_interm::{Stage, TraceDBInterm};

const PATH_WRITE_BATCH: usize = 1000;

pub enum DataInput {
    StateDB(String),
}

impl DataInput {
    pub fn path(&self) -> String {
        match self {
            DataInput::StateDB(p) => { p.clone() }
        }
    }
}

pub fn gen_case(
    data_input: DataInput,
    data_output: String,
    dict: HashMap<String, Value>,
    opt_intermediate_path: Option<String>,
    sqlite_cache_size: Option<u64>,
    initialize_setup: bool,
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
        DataInput::StateDB(path) => {
            let graph = state_db_to_from_action(path.clone(), dict, intermediate.clone(), sqlite_cache_size)?;
            let duration = inst.elapsed();
            info!("Time elapsed to read from state DB {} and write actions, time costs: {:?}", path, duration);

            let inst = Instant::now();
            action_graph_output_to_db(&graph, intermediate.clone(), sqlite_cache_size)?;
            let duration = inst.elapsed();
            info!("Time elapsed to generate path, time costs: {:?}",  duration);
        }
    }


    let inst = Instant::now();
    let opt = OptBuild {
        initialize_setup,
        sqlite_cache_size,
    };
    TraceBuilder::build(intermediate, data_output, opt)?;

    let duration = inst.elapsed();
    info!("Time elapsed to gen final trace, time costs: {:?}", duration);
    Ok(())
}


fn action_graph_output_to_db(
    action_graph: &ActionGraph<i64>,
    db_path: String,
    sqlite_cache_size: Option<u64>,
) -> Res<()> {
    let db = RefCell::new(TraceDBInterm::new(db_path, None, sqlite_cache_size)?);
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
        action_graph.build_path( &f_write_path)?;
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
