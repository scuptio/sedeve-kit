use std::collections::HashMap;
use std::path::PathBuf;

use rusqlite::{Connection, params, Statement, Transaction};
use scupt_util::error_type::ET;

use scupt_util::res::Res;
use scupt_util::res_of::{res_parse, res_sqlite};
use serde_json::Value;

use uuid::Uuid;


pub struct TraceDB {
    conn:Connection,
}

pub struct TraceDBTrans<'c> {
    transaction: Transaction<'c>,
    stmt_insert_path: Option<Statement<'c>>,
    stmt_insert_action:Option<Statement<'c>>,
    stmt_select_path:Option<Statement<'c>>,
    stmt_select_action:Option<Statement<'c>>,
    stmt_insert_trace:Option<Statement<'c>>,
    stmt_select_trace:Option<Statement<'c>>,
}


impl TraceDB {
    pub fn new(db_path:String) -> Res<Self> {
        let path_buf = PathBuf::from(db_path.clone());
        let r_conn = Connection::open(path_buf);
        let conn = res_sqlite(r_conn)?;
        let db = Self {
            conn,
        };
        Ok(db)
    }

    pub fn new_trans(&mut self, create_db:bool, read_only:bool) -> Res<TraceDBTrans> {
        TraceDBTrans::new(&mut self.conn, create_db, read_only)
    }

    pub fn vacuum(&self) -> Res<()> {
        let r = self.conn.execute("vacuum;", ());
        res_sqlite(r)?;
        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}


impl <'c> TraceDBTrans<'c> {
    fn new(conn: &'c mut Connection, create_db:bool, read_only:bool) -> Res<Self> {
        let r = conn.transaction();
        let trans = res_sqlite(r)?;
        let mut db = Self {
            transaction: trans,
            stmt_insert_path: None,
            stmt_insert_action: None,
            stmt_select_path: None,
            stmt_select_action: None,
            stmt_insert_trace: None,
            stmt_select_trace: None,
        };
        if create_db {
            db.create()?;
        }
        db.prepare(read_only)?;
        Ok(db)
    }


    fn create(&mut self) -> Res<()> {
        // create tables
        let r = self.transaction.execute(
            "drop table if exists action;",
            (), // empty list of parameters.
        );
        res_sqlite(r)?;

        let r = self.transaction.execute(
            "drop table if exists path;",
            (), // empty list of parameters.
        );
        res_sqlite(r)?;

        let r = self.transaction.execute(
            "drop table if exists trace;",
            (), // empty list of parameters.
        );
        res_sqlite(r)?;

        let r = self.transaction.execute(
            "create table action (
                        id text not null,
                        action_json text not null
                        )",
                (), // empty list of parameters.
        );
        res_sqlite(r)?;

        let r = self.transaction.execute(
            "create table path (
                        id text not null,
                        path_json text not null
                        )",
            (), // empty list of parameters.
        );
        let _ = res_sqlite(r)?;

        let r = self.transaction.execute(
            "create table trace (
                        id text not null primary key,
                        path_json text not null,
                        trace_json text not null
                        )",
            (),
        );
        let _ = res_sqlite(r)?;

        return Ok(());
    }

    fn prepare(&mut self, read:bool) -> Res<()> {
        let t:*mut Transaction  = &mut self.transaction;
        unsafe {
            self.prepare_for_connection(&(*t), read)?;
        }
        Ok(())
    }

    fn prepare_for_connection(&mut self, connection:&'c Connection, read:bool) -> Res<()> {
        if !read {
            {
                let sql = "insert into path (id, path_json) VALUES (?1, ?2)";
                let r = connection.prepare(sql);
                let stmt = r.unwrap();
                let mut opt = Some(stmt);
                std::mem::swap(&mut self.stmt_insert_path, &mut opt);
            }
            {
                let sql = "insert into action (id, action_json) values (?1, ?2)";
                let r = connection.prepare(sql);
                let stmt = r.unwrap();
                let mut opt = Some(stmt);
                std::mem::swap(&mut self.stmt_insert_action, &mut opt);
            }
            {
                let sql = "insert into trace (id, path_json, trace_json) values (?1, ?2, ?3);";
                let r = connection.prepare(sql);
                let stmt = r.unwrap();
                let mut opt = Some(stmt);
                std::mem::swap(&mut self.stmt_insert_trace, &mut opt);
            }

            {
                let sql = "select id, path_json from path";
                let r = connection.prepare(sql);
                let stmt = r.unwrap();
                let mut opt = Some(stmt);
                std::mem::swap(&mut self.stmt_select_path, &mut opt);
            }

            {
                let sql = "select id, action_json from action";
                let r = connection.prepare(sql);
                let stmt = r.unwrap();
                let mut opt = Some(stmt);
                std::mem::swap(&mut self.stmt_select_action, &mut opt);
            }
        } else {
            {
                let sql = "select trace_json from trace order by id";
                let r = connection.prepare(sql);
                let stmt = r.unwrap();
                let mut opt = Some(stmt);
                std::mem::swap(&mut self.stmt_select_trace, &mut opt);
            }
        }

        Ok(())
    }

    pub fn commit(self) -> Res<()> {
        res_sqlite(self.transaction.commit())
    }

    pub fn insert_path( &mut self, path:Vec<String>) -> Res<()> {
        let opt_r = serde_json::to_value(path);
        let s = res_parse(opt_r)?;
        let id = Uuid::new_v4();
        let stmt  = Self::opt_stmt(&mut self.stmt_insert_path)?;
        let r = stmt.execute(params!(id.to_string(), s.to_string()));
        let _ = res_sqlite(r)?;
        Ok(())
    }

    pub fn insert_action( &mut self, id:String, action:Value) -> Res<()> {
        let stmt  = Self::opt_stmt(&mut self.stmt_insert_action)?;
        let r = stmt.execute(params!(id.to_string(), action.to_string()));
        let _ = res_sqlite(r);
        Ok(())
    }

    pub fn insert_trace( &mut self, id:String, path:Vec<String>, trace:String) -> Res<()> {
        let opt_r = serde_json::to_value(path);
        let s = res_parse(opt_r)?;
        let stmt  = Self::opt_stmt(&mut self.stmt_insert_trace)?;
        let r = stmt.execute(params!(id.to_string(), s.to_string(), trace));
        let _ = res_sqlite(r)?;
        Ok(())
    }

    pub fn trace(&mut self) -> Res<Vec<String>> {
        let stmt = Self::opt_stmt(&mut self.stmt_select_trace)?;
        let mut path_vec = vec![];
        let r_rows = stmt.query([]);
        let mut rows = res_sqlite(r_rows)?;

        while let Some(row) = res_sqlite(rows.next())? {
            let json_value: String = res_sqlite(row.get(0))?;
            path_vec.push(json_value);
        }
        Ok(path_vec)
    }

    pub fn path_vec(&mut self) -> Res<Vec<(String, Vec<String>)>> {
        let stmt = Self::opt_stmt(&mut self.stmt_select_path)?;
        let mut path_vec = vec![];
        let r_rows = stmt.query([]);
        let mut rows = res_sqlite(r_rows)?;

        while let Some(row) = res_sqlite(rows.next())? {
            let id: String = res_sqlite(row.get(0))?;
            let json_value: String = res_sqlite(row.get(1))?;
            let r = serde_json::from_str(json_value.as_str());
            let vec: Vec<String> = match r {
                Ok(value) => { value }
                Err(e) => { return Err(ET::JSONError(e.to_string())); }
            };
            path_vec.push((id, vec));
        }
        Ok(path_vec)
    }


    pub fn action_map(
        &mut self,
    ) -> Res<HashMap<String, String>>
    {
        let mut map = HashMap::new();
        let statement = Self::opt_stmt(&mut self.stmt_select_action)?;
        let r_rows = statement.query([]);
        let mut rows = res_sqlite(r_rows)?;
        while let Some(row) = res_sqlite(rows.next())? {
            let id = res_sqlite(row.get(0))?;
            let json_value: String = res_sqlite(row.get(1))?;
            map.insert(id, json_value);
        }
        Ok(map)
    }

    pub fn drop_intermediate_table(&self) -> Res<()> {
        // create tables
        let r = self.transaction.execute(
            "drop table if exists action;",
            (), // empty list of parameters.
        );
        res_sqlite(r)?;

        let r = self.transaction.execute(
            "drop table if exists path;",
            (), // empty list of parameters.
        );
        res_sqlite(r)?;
        Ok(())
    }
    fn opt_stmt<'a, 'b>(opt: &'a mut Option<Statement<'b>>) -> Res<&'a mut Statement<'b>> {
        match opt {
            Some(t) => { Ok(t) }
            None => { Err(ET::NoneOption)}
        }
    }
}