use std::sync::Arc;

use scupt_util::res::Res;

use crate::action_player::action_incoming::ActionIncoming;
use crate::action_player::action_incoming_factory::ActionIncomingFactory;
use crate::trace_gen::trace_db::TraceDB;

pub struct TraceReader {}


impl TraceReader {

    /// Reads F-exec from the given file path SQLite DB and returns them as a vector of
    /// `Arc<dyn ActionIncoming<Input, Output, Internal>>`.
    ///
    /// # Arguments
    ///
    /// * `path` - A `String` representing the file path of the SQLite DB.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `Arc<dyn ActionIncoming<Input, Output, Internal>>`
    /// if the operation was successful, or an error if it failed.
    ///
    /// # Examples
    ///
    ///
    pub fn read_action(
        path: String,
    ) -> Res<Vec<Arc<dyn ActionIncoming>>>
    {
        let mut db = TraceDB::new(path)?;
        let mut trans = db.new_trans(false, true)?;
        let trace_vec = trans.trace()?;
        let mut vec = vec![];
        for json_value in trace_vec {
            let incoming: Arc<dyn ActionIncoming> =
                ActionIncomingFactory::action_incoming_from_string(json_value)?;
            vec.push(incoming);
        }
        Ok(vec)
    }
}

