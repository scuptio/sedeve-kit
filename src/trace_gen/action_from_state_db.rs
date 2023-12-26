use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;

use rusqlite::Connection;
use scupt_util::message::MsgTrait;
use scupt_util::res::Res;
use scupt_util::res_of::{res_parse, res_sqlite};
use serde_json::Value;
use tracing::info;

use crate::action::action_message::ActionMessage;
use crate::action::tla_actions::TLAActionSeq;
use crate::action::tla_typed_value::get_typed_value;
use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::read_json::read_from_dict_json;
use crate::trace_gen::trace_db_interm::{Stage, TraceDBInterm};

pub fn read_actions<F>(path:String, dict: &HashMap<String, Value>, fn_handle_action:&F)
                       -> Res<()>
    where F: Fn(Value) -> Res<()>,
{
    let conn = res_sqlite(Connection::open(path))?;
    let mut stmt = res_sqlite(conn.prepare("select json_string from state order by json_string;"))?;
    let mut rows = res_sqlite(stmt.query([]))?;
    while let Some(row) = res_sqlite(rows.next())? {
        let json:String = res_sqlite(row.get(0))?;
        let value : Value = res_parse(serde_json::from_str(json.as_str()))?;
        let value = get_typed_value(value, dict)?;
        fn_handle_action(value)?;
    }
    Ok(())
}

pub fn read_action_message<M: MsgTrait + 'static, F>(
    path_db: String,
    path_map: String,
    f : &F
) -> Res<()>
    where F: Fn(ActionMessage<M>) -> Res<()>
{
    let map = read_from_dict_json(Some(path_map.clone())).unwrap();
    let f = |v: Value| -> Res<()> {
        let tla_action_seq = TLAActionSeq::from(v.clone())?;
        for vec in [tla_action_seq.actions(), tla_action_seq.states()] {
            for a in vec {
                let j = a.to_action_json()?;
                let s = j.to_action_message();
                let m: serde_json::Result<ActionMessage<M>> = serde_json::from_str(s.to_string().unwrap().as_str());
                if m.is_err() {
                    eprintln!("==== error action sequence ==== : \n{}", serde_json::to_string_pretty(&v).unwrap());
                    eprintln!("==== error action ==== : \n{}", s.to_string().unwrap())
                }
                f(m.unwrap())?;
            }
        }
        Ok(())
    };
    read_actions(path_db.to_string(), &map, &f).unwrap();
    Ok(())
}

fn read_action_batch<F>(
    path: String,
    batch_rows: u64,
    dict: &HashMap<String, Value>,
    fn_handle_action: &F)
    -> Res<()>
    where F: Fn(Vec<Value>) -> Res<()>,
{
    let batch_rows = batch_rows as usize;
    let conn = res_sqlite(Connection::open(path))?;
    let mut stmt = res_sqlite(conn.prepare("select json_string from state order by json_string;"))?;
    let mut rows = res_sqlite(stmt.query([]))?;
    let mut batch = Vec::with_capacity(batch_rows);
    while let Some(row) = res_sqlite(rows.next())? {
        let json: String = res_sqlite(row.get(0))?;
        let value: Value = res_parse(serde_json::from_str(json.as_str()))?;
        let value = get_typed_value(value, dict)?;
        batch.push(value);
        if batch.len() >= batch_rows {
            fn_handle_action(batch)?;
            batch = Vec::with_capacity(batch_rows)
        }
    }

    if !batch.is_empty() {
        fn_handle_action(batch)?;
    }

    Ok(())
}

pub fn graph_read_actions_from_state_db(path: String, dict: HashMap<String, Value>, output_path: String) -> Res<ActionGraph<i64>> {
    let db = RefCell::new(TraceDBInterm::new(output_path, None, None)?);
    let stage = {
        let db_ref = db.borrow();
        let stage = db_ref.get_state()?;
        match &stage {
            Stage::WriteAction => {
                db_ref.begin_write_action()?;
                stage
            },

            _ => {
                stage
            }
        }
    };
    if stage == Stage::WriteAction {
        info!("To write actions to DB");
        let f = |vec: Vec<Value>| -> Res<()> {
            let mut vec_rows: Vec<(i64, i64, String, String)> = vec![];
            let db_ref = db.borrow();
            for v in vec {
                let seq = TLAActionSeq::from(v)?;
                vec_rows.push(seq.to_tuple()?);
            }
            db_ref.write_action(vec_rows)?;
            Ok(())
        };

        let inst = Instant::now();
        read_action_batch(path, 10000, &dict, &f)?;
        let duration = inst.elapsed();
        {
            let db_ref = db.borrow();
            db_ref.end_write_action()?;
        }
        info!("Time elapsed to write actions to DB, time costs: {:?}", duration);
    }

    let db_ref = db.borrow();
    let graph = db_ref.gen_graph()?;

    Ok(graph)
}