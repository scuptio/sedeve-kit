use std::sync::Mutex;

use rusqlite::Connection;
use scupt_util::res::Res;
use scupt_util::res_of::res_sqlite;
use serde_json::Value;

/// sqlite database stores trace
pub struct TraceDB {
    _path: String,
    conn: Mutex<Connection>,
}



impl TraceDB {
    pub fn new(path: String) -> Res<Self> {
        let r_conn = Connection::open(path.clone());
        let mut conn = res_sqlite(r_conn)?;
        {
            let tran = res_sqlite(conn.transaction())?;
            let r = tran.execute(
                r#"create table if not exists trace (
                        id text not null,
                        trace_json text not null
                        )"#, ());
            res_sqlite(r)?;
            tran.commit().unwrap();
        }
        let r = Self {
            _path: path,
            conn: Mutex::new(conn),
        };
        Ok(r)
    }

    pub fn write_trace(&self, traces: Vec<(String, Vec<Value>)>) -> Res<()> {
        let sql = "insert into trace (id, trace_json) values (?1, ?2);";
        let mut conn = self.conn.lock().unwrap();
        {
            let trans = conn.transaction().unwrap();
            {
                let mut stmt = trans.prepare(sql).unwrap();
                for (id, trace) in traces {
                    let trace_json = serde_json::to_string_pretty(&trace).unwrap();
                    stmt.execute((id, trace_json)).unwrap();
                }
            }
            trans.commit().unwrap();
        }
        Ok(())
    }

    pub fn read_trace(&self) -> Res<Vec<String>> {
        let sql = "select trace_json from trace order by id";
        let mut conn = self.conn.lock().unwrap();
        let trans = conn.transaction().unwrap();
        let mut stmt = trans.prepare(sql).unwrap();
        let mut rows = stmt.query(()).unwrap();
        let mut vec = vec![];
        while let Some(row) = res_sqlite(rows.next())? {
            let json_value: String = res_sqlite(row.get(0))?;
            vec.push(json_value);
        }
        Ok(vec)
    }
}