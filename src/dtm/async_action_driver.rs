use async_trait::async_trait;
use scupt_util::node_id::NID;
use scupt_util::res::Res;

use crate::action::action_type::{ActionBeginEnd, ActionType};

#[async_trait]
pub trait AsyncActionDriver: Sync + Send {
    async fn action(
        &self,
        action_type:ActionType,
        action_begin_end: ActionBeginEnd,
        source_node_id:NID,
        dest_node_id:NID,
        action_json_str: String
    ) -> Res<()>;
}
