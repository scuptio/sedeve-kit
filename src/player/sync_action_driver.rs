use scupt_util::node_id::NID;
use scupt_util::res::Res;

use crate::action::action_type::ActionType;

// TODO, SyncActionDriver implement
pub trait SyncActionDriver: Sync + Send {
    async fn begin_action(&self, action_type:ActionType,source:NID, dest:NID, action: String) -> Res<()>;

    async fn end_action(&self, action_type:ActionType, source:NID, dest:NID, action: String) -> Res<()>;
}
