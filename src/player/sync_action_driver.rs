use scupt_util::node_id::NID;
use scupt_util::res::Res;

use crate::action::action_type::ActionType;


pub trait SyncActionDriver: Sync + Send {
    fn begin_action(&self, action_type:ActionType,source:NID, dest:NID, action: String) -> Res<()>;

    fn end_action(&self, action_type:ActionType, source:NID, dest:NID, action: String) -> Res<()>;
}
