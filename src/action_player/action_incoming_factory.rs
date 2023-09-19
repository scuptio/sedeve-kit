
use std::sync::Arc;

use scupt_util::res::Res;

use crate::action_player::action_incoming::ActionIncoming;
use crate::action_player::action_input_json::ActionInputJson;

pub struct ActionIncomingFactory{

}


impl ActionIncomingFactory {
    pub fn action_incoming_from_json_file(path: String) -> Res<Arc<dyn ActionIncoming>> {
        let ai = ActionInputJson::from_file(path)?;
        Ok(Arc::new(ai))
    }

    pub fn action_incoming_from_string(json_string: String) -> Res<Arc<dyn ActionIncoming>> {
        let ai = ActionInputJson::from_json_string(json_string)?;
        Ok(Arc::new(ai))
    }
}