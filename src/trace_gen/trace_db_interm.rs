use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;

use std::sync::Mutex;
use lazy_static::lazy_static;

use rusqlite::{Connection, OptionalExtension};
use rusqlite::ffi::{sqlite3_mprintf, sqlite3_temp_directory};
use scupt_util::res::Res;
use scupt_util::res_of::{res_parse, res_sqlite};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::trace_gen::action_graph::ActionGraph;
use crate::trace_gen::graph_util::adj_add_new_edge;

/// intermediate database to generate traces
pub struct TraceDBInterm {
    _path: String,
    conn: Mutex<Connection>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Stage {
    WriteAction,
    GeneratePath,
    GenerateTrace,
}

lazy_static! {
    static ref TEMP_PATH: Mutex<String> = Default::default();
}

impl TraceDBInterm {
    pub fn new(
        path: String,
        temp_path:Option<String>,
        cache_size:Option<u64>
    ) -> Res<Self> {
        if let Some(temp) = temp_path {
            let mut guard = TEMP_PATH.lock().unwrap();
            *guard = temp;
            unsafe {
                let s = CString::new(guard.as_str()).unwrap();
                sqlite3_temp_directory = sqlite3_mprintf(s.as_ptr());
            }
        }
        let r_conn = Connection::open(path.clone());
        let mut conn = res_sqlite(r_conn)?;
        if let Some(size) = cache_size {
            let _r = conn.pragma_update(None, "cache_size", size);
            res_sqlite(_r)?;
        }

        let tran = res_sqlite(conn.transaction())?;
        let r = tran.execute(
            r#"create table if not exists action (
                     id long not null primary key,
                     prev_id long not null,
                     state_json text not null,
                     action_json text not null
                 )"#, ());
        res_sqlite(r)?;

        let r = tran.execute(
            r#"create table if not exists meta (
                     key text not null primary key,
                     val text not null
                 )"#, ());
        res_sqlite(r)?;

        let r = tran.execute(
            r#"create table if not exists path (
                     id text not null,
                     seq long not null,
                     action_id long not null
                 )"#, ());
        res_sqlite(r)?;
        res_sqlite(tran.commit())?;

        let r = Self {
            _path: path,
            conn: Mutex::new(conn),
        };

        Ok(r)
    }

    pub fn get_state(&self) -> Res<Stage> {
        let conn = self.conn.lock().unwrap();

        let r_state: rusqlite::Result<Option<String>> = conn.query_row(
            "select val from  meta where key = ?1",
            ("state".to_string(), ),
            |r| r.get(0),
        ).optional();
        let opt_state_str = res_sqlite(r_state)?;
        match opt_state_str {
            Some(state_str) => {
                let stage: Stage = serde_json::from_str(state_str.as_str()).unwrap();
                Ok(stage)
            }
            None => {
                Ok(Stage::WriteAction)
            }
        }
    }

    pub fn set_state(&self, state: Stage) -> Res<()> {
        let mut conn = self.conn.lock().unwrap();
        let r_tran = conn.transaction();
        let trans = res_sqlite(r_tran)?;
        let val = serde_json::to_string(&state).unwrap();
        let s = trans.execute("
            insert or replace into meta(key, val) values(?1, ?2)", ("state".to_string(), val));
        let _ = res_sqlite(s)?;
        let r = trans.commit();
        res_sqlite(r)?;
        Ok(())
    }

    pub fn write_path(&self, batch: Vec<Vec<i64>>) -> Res<()> {
        let sql = "insert into path (
                        id,
                        seq,
                        action_id
                        ) values (?1, ?2, ?3)";
        let mut conn = self.conn.lock().unwrap();
        let r_tran = conn.transaction();
        let tran = res_sqlite(r_tran)?;
        {
            let mut stmt = res_sqlite(tran.prepare(sql))?;
            for path in batch {
                let id = uuid::Uuid::new_v4().to_string();
                for (i, action_id) in path.iter().rev().enumerate() {
                    let r = stmt.execute((id.clone(), (i + 1) as i32, *action_id));
                    let _ = res_sqlite(r)?;
                }
            }
        }
        let _r = tran.commit();
        let _ = res_sqlite(_r)?;
        Ok(())
    }
    pub fn write_action(&self, batch: Vec<(i64, i64, String, String)>) -> Res<()> {
        let mut conn = self.conn.lock().unwrap();
        let r_tran = conn.transaction();
        let tran = res_sqlite(r_tran)?;
        {
            let sql1 = "insert into action(\
                id, prev_id, state_json, action_json\
                    ) values (\
                ?1, ?2, ?3, ?4)\
                on conflict (id) do nothing\
                ";
            let r_stmt1 = tran.prepare(sql1);
            let mut stmt1 = res_sqlite(r_stmt1)?;
            for (id, prev_id, state_json, action_json) in batch {
                let _ = stmt1.insert((id, prev_id, state_json, action_json));
            }
        }
        let r = tran.commit();
        res_sqlite(r)?;
        Ok(())
    }

    fn truncate_table(&self, table_name: String) -> Res<()> {
        let mut conn = self.conn.lock().unwrap();
        let r_tran = conn.transaction();
        let tran = res_sqlite(r_tran)?;
        {
            let sql1 = format!("delete from {};", table_name);
            let r_stmt1 = tran.prepare(sql1.as_str());
            let mut stmt1 = res_sqlite(r_stmt1)?;
            let _ = res_sqlite(stmt1.execute(()))?;
        }
        let _r = res_sqlite(tran.commit());
        Ok(())
    }

    pub fn begin_write_action(&self) -> Res<()> {
        self.truncate_table("action".to_string())?;
        self.set_state(Stage::WriteAction)?;
        Ok(())
    }

    pub fn end_write_action(&self) -> Res<()> {
        Ok(())
    }

    pub fn begin_generate_path(&self) -> Res<()> {
        self.truncate_table("path".to_string())?;
        self.set_state(Stage::GeneratePath)?;
        Ok(())
    }

    pub fn end_generate_path(&self) -> Res<()> {
        self.set_state(Stage::GenerateTrace)?;
        Ok(())
    }

    pub fn read_adj<F>(&self, f: &F, opt_batch: Option<u64>) -> Res<()>
        where F: Fn(Vec<(i64, i64)>) {
        let conn = self.conn.lock().unwrap();
        let r_stmt = conn.prepare(
            "select id, prev_id from action"
        );
        let mut stmt = res_sqlite(r_stmt)?;
        let r_rows = stmt.query(());
        let mut rows = res_sqlite(r_rows)?;
        let mut cnt: usize = 0;
        let mut vec: Vec<(i64, i64)> = vec![];
        while let Some(row) = res_sqlite(rows.next())? {
            let id: i64 = res_sqlite(row.get(0))?;
            let id_prev: i64 = res_sqlite(row.get(1))?;
            vec.push((id, id_prev));
            if let Some(batch) = opt_batch {
                if batch as usize >= cnt {
                    f(vec);
                    vec = vec![];
                }
            }
            cnt += 1;
        }
        f(vec);
        Ok(())
    }

    pub fn state<F>(&self, f_handle_state:&F) -> Res<()>
        where F:Fn(String, Vec<Value>)
    {
        let sql = r#"select id, state_json from action;"#;
        let conn = self.conn.lock().unwrap();
        let stmt_r = conn.prepare(sql);
        let mut stmt = res_sqlite(stmt_r)?;
        let mut rows = res_sqlite(stmt.query(()))?;
        while let Some(row) = res_sqlite(rows.next())? {
            let id:i64 = res_sqlite(row.get(0))?;
            let state_json: String = res_sqlite(row.get(1))?;
            let state_value: Value = res_parse(serde_json::from_str(state_json.as_str()))?;
            let vec = match  state_value {
                Value::Array(vec) => {
                    vec
                }
                _  => {
                    vec![state_value]
                }
            };
            f_handle_state(id.to_string(), vec);
        }
        Ok(())
    }

    pub fn trace<F>(&self, f_handle_trace: &F) -> Res<()>
        where F: Fn(String, Vec<Value>)
    {
        let sql = r#"
        select path.id path_id,
            action.state_json, action.action_json, row_number()
            over (partition by path.id order by path.seq)
        from action join path on action.id = path.action_id;"#;

        let conn = self.conn.lock().unwrap();
        let stmt_r = conn.prepare(sql);
        let mut stmt = res_sqlite(stmt_r)?;
        let mut rows = res_sqlite(stmt.query(()))?;
        let mut states: Vec<Value> = vec![];
        let mut actions: Vec<Value> = vec![];
        let mut opt_id: Option<String> = None;
        while let Some(row) = res_sqlite(rows.next())? {
            let path_id = res_sqlite(row.get(0))?;
            if let Some(prev_path_id) = &opt_id {
                if *prev_path_id != path_id {
                    let mut trace = states;
                    trace.append(&mut actions);
                    f_handle_trace(prev_path_id.clone(), trace);
                    actions = vec![];
                    states = vec![];
                    opt_id = None;
                }
            }

            let state_json: String = res_sqlite(row.get(1))?;
            let action_json: String = res_sqlite(row.get(2))?;
            let action_value: Value = res_parse(serde_json::from_str(action_json.as_str()))?;
            let mut action_vec = match action_value {
                Value::Array(a) => a,
                _ => { panic!("must be an array") }
            };
            actions.append(&mut action_vec);
            if opt_id.is_none() {
                let state_value: Value = res_parse(serde_json::from_str(state_json.as_str()))?;
                let mut state_vec = match state_value {
                    Value::Array(a) => a,
                    _ => { panic!("must be an array") }
                };
                states.append(&mut state_vec);
            }
            opt_id = Some(path_id);
        }
        Ok(())
    }

    pub fn gen_graph(&self) -> Res<ActionGraph<i64>> {
        let adj = RefCell::new(HashMap::new());
        let stage = self.get_state()?;
        if stage == Stage::WriteAction || stage == Stage::GeneratePath {
            let handle_adj = |vec: Vec<(i64, i64)>| {
                let mut adj_ref = adj.borrow_mut();
                for (u, v) in vec {
                    adj_add_new_edge(&mut adj_ref, &u, &v);
                }
            };
            self.read_adj(&handle_adj, None)?;
        }
        Ok(ActionGraph::new(adj.into_inner()))
    }
}