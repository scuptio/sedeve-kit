use std::sync::Arc;

use scupt_util::res::Res;

use crate::player::action_incoming::ActionIncoming;
use crate::player::action_input_json::ActionInputJson;

pub struct ActionIncomingFactory{

}


/// Factory class to build ActionIncoming
impl ActionIncomingFactory {
    /// Create an ActionIncoming from a JSON file
    pub fn action_incoming_from_json_file(path: String) -> Res<Arc<dyn ActionIncoming>> {
        let ai = ActionInputJson::from_file(path)?;
        Ok(Arc::new(ai))
    }

    /// Create an ActionIncoming from a JSON string
    pub fn action_incoming_from_string(json_string: String) -> Res<Arc<dyn ActionIncoming>> {
        let ai = ActionInputJson::from_json_string(json_string)?;
        Ok(Arc::new(ai))
    }
}