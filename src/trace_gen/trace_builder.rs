// convert TLA+ output json to action serde json


use std::cell::RefCell;
use std::path::PathBuf;
use std::str::FromStr;
use scupt_util::error_type::ET::IOError;

use scupt_util::res::Res;
use serde_json::Value;
use tracing::info;

use crate::trace_gen::trace_db::TraceDB;
use crate::trace_gen::trace_db_interm::TraceDBInterm;

pub struct TraceBuilder {

}

#[derive(Clone)]
pub struct OptBuild {
    pub initialize_setup:bool
}

impl Default for OptBuild {
    fn default() -> Self {
        Self {
            initialize_setup: false,
        }
    }
}
impl TraceBuilder {
    /// This function converts TLA+ __action__ variables that have been dumped from the Toolbox and
    /// stored in the input SQLite database to serialized structs using serde_json. The resulting
    /// structs are then stored in the output SQLite database.
    ///
    /// # Arguments
    ///
    /// * `db_path` - A `String` representing the file path to the SQLite database.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the operation was successful or not.
    ///
    /// # Examples
    ///
    ///
    pub fn build(
        db_input: String,
        db_output: String,
        opt_build: OptBuild
    ) -> Res<()> {
        Self::build_gut(db_input, db_output, 10000, opt_build)?;
        Ok(())
    }


    fn build_gut(
        path_db_input: String,
        path_db_output: String,
        batch: u64,
        opt_build_trace: OptBuild
    ) -> Res<()>
    {
        info!("To write traces to DB");
        let temp_dir = {
            let path_buf = PathBuf::from_str(path_db_output.as_str()).unwrap();
            let path = path_buf.parent().unwrap();
            if !(path.exists() && path.is_dir()) {
                return Err(IOError("folder not exists".to_string()));
            }
            path.to_str().unwrap().to_string()
        };
        let db_input = TraceDBInterm::new(
            path_db_input,
            Some(temp_dir),
            Some(20*1024*1024), // 10GB
        )?;
        let db_output: RefCell<TraceDB> = RefCell::new(TraceDB::new(path_db_output)?);
        let traces: RefCell<Vec<(String, Vec<Value>)>> = RefCell::new(vec![]);
        let num :RefCell<u64> = RefCell::new(0);

        let f_write = || {
            let mut traces_ref = traces.borrow_mut();
            let db_ref = db_output.borrow();
            let mut vec_trace = vec![];
            std::mem::swap(&mut vec_trace, &mut traces_ref);
            let mut num_ref = num.borrow_mut();
            *num_ref += vec_trace.len() as u64;
            info!("write {} traces", *num_ref);
            db_ref.write_trace(vec_trace).unwrap();
        };
        let f = |id, v| {
            let write = {
                let mut traces_ref = traces.borrow_mut();
                traces_ref.push((id, v));
                traces_ref.len() == batch as usize
            };
            if write {
                f_write();
            }
        };
        let initialize_setup = opt_build_trace.initialize_setup;
        if initialize_setup {
            db_input.state(&f)?;
        } else {
            db_input.trace(&f)?;
        }
        f_write();
        Ok(())
    }
}

