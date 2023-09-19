// convert TLA+ output json to action serde json


use scupt_util::res::Res;
use scupt_util::res_of::res_option;
use serde_json::Value;

use tracing::trace;
use uuid::Uuid;
use crate::action::tla_actions::TLAActionSeq;
use crate::trace_gen::trace_db::{TraceDB, TraceDBTrans};

pub struct TraceBuilder {

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
        db_path: String,
        remove_intermediate:bool
    ) -> Res<()> {
        Self::build_gut(db_path, remove_intermediate)
    }

    fn add_trace_to_db(trans:&mut TraceDBTrans, id:String, trace_path:Vec<i64>, trace:Vec<Value>) -> Res<()> {
        if !trace.is_empty() {
            let s = serde_json::to_string_pretty(&trace).unwrap();
            trans.insert_trace(id.to_string(), trace_path.clone(), s)?;
        }
        Ok(())
    }

    fn build_gut(
        db_path:String,
        remove_intermediate:bool
    ) -> Res<()>
    {
        let mut db = TraceDB::new(db_path.clone())?;
        let mut trans = db.new_trans(false, false)?;

        let vec = trans.path_vec()?;
        let map = trans.action_map()?;

        trace!("{:?}", vec);

        for (_id, path_vec) in vec.iter() {
            let mut trace = vec![];
            let mut path = vec![];

            for i in 0..path_vec.len() {
                let v_id = path_vec[i];

                let opt = map.get(&v_id);
                let s = res_option(opt)?;
                let tla_action_seq = TLAActionSeq::from_str(s)?;
                let continuous_trace = if i > 0 {
                    let v_id1 = path_vec[i - 1];
                    let opt1 = map.get(&v_id1);
                    let s1 = res_option(opt1)?;
                    let tla_action_seq1 = TLAActionSeq::from_str(s1)?;
                    tla_action_seq1.id.eq(&tla_action_seq.id_prev)
                } else {
                    true
                };
                if continuous_trace {
                    path.push(v_id);
                    for action in tla_action_seq.actions() {
                        let json_value = action.to_action_json()?;
                        trace.push(json_value.action_json_value());
                    }
                    trace!("id: {} {:?}", v_id, vec);
                } else {
                    // finish this trace and add it to database
                    let trace_id = Uuid::new_v4();
                    Self::add_trace_to_db(&mut trans, trace_id.to_string(), path, trace)?;

                    // create a new trace
                    path = vec![];
                    trace = vec![];
                    if i + 1 != path_vec.len() { // not the last one
                        path.push(v_id);
                        for action in tla_action_seq.states() {
                            // a new trace, initialize state
                            let json_value = action.to_action_json()?;
                            trace.push(json_value.action_json_value());
                        }
                        for action in tla_action_seq.actions() {
                            // the following actions
                            let json_value = action.to_action_json()?;
                            trace.push(json_value.action_json_value());
                        }
                    }
                }
            }
            let trace_id = Uuid::new_v4();
            Self::add_trace_to_db(&mut trans, trace_id.to_string(), path, trace)?;
        }

        trans.commit()?;


        if remove_intermediate {
            let trans = db.new_trans(false, false)?;
            trans.drop_intermediate_table()?;
            trans.commit()?;
        }

        db.vacuum()?;
        Ok(())
    }
}

